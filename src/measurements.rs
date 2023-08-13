//! Sensor Measurements.

use core::f32::consts::FRAC_PI_2;

use crate::math::{CoordsCartesian, CoordsSpherical};

/// The recognized hand state.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HandState {
    /// No hand was found.
    HandNotFound,
    /// Hand was found with this position.
    HandFound {
        /// The hand position in spherical coordinates.
        hand_pos: CoordsSpherical,
    },
}

/// Configurable sensor parameters. Different for every sensor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SensorParams {
    /// The horizontal FOV of the sensor.
    pub fov_horizontal: f32,
    /// The vertical FOV of the sensor.
    pub fov_vertical: f32,
}

impl SensorParams {
    /// The default parameters for the ST VL53L5CX TOF-Sensor.
    pub fn default_vl53l5cx() -> Self {
        Self {
            // The VL53L5CX has a diagonal FOV of 63deg, so fov_x = fov_y = 63.0 / sqrt(2) = 45.0
            fov_horizontal: 45.0,
            fov_vertical: 45.0,
        }
    }
}

/// Represents a sensor measurement coming from the TOF sensor.
///
/// Expects that the zones are already rotated and mirrored,
/// so that the zone with index \[0\]\[0\] is the top left corner when looking at the sensor.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SensorMeasurement<const RES_X: usize, const RES_Y: usize> {
    /// The measured distances of each zone.
    ///
    /// Invalid distance measurements are represented by value -1.0.
    pub zone_dist: [[f32; RES_X]; RES_Y],
    /// The time of the measurement in milliseconds. Must be monotonically increasing.
    pub time_ms: u32,
}

impl<const RES_X: usize, const RES_Y: usize> SensorMeasurement<RES_X, RES_Y> {
    /// Creates a new measurement.
    pub fn new(zone_dist: [[f32; RES_X]; RES_Y]) -> Self {
        Self {
            zone_dist,
            time_ms: 0,
        }
    }

    /// An invalid measurement. The time is set to zero, distances are set to invalid (value `-1.0`).
    pub fn invalid() -> Self {
        Self {
            zone_dist: [[-1.0; RES_X]; RES_Y],
            time_ms: 0,
        }
    }

    /// Finds the position in the matrix and distance value of the zone with minimal distance.
    ///
    /// Returns the tuple: (["x-pos in matrix", "y-pos in matrix"], "distance").
    pub(crate) fn min_dist(&self) -> ([usize; 2], f32) {
        self.zone_dist
            .into_iter()
            .enumerate()
            .flat_map(|(y_pos, r)| {
                r.into_iter().enumerate().filter_map(move |(x_pos, d)| {
                    if d > 0.0 {
                        Some(([x_pos, y_pos], d))
                    } else {
                        None
                    }
                })
            })
            .fold(([0, 0], f32::MAX), |(acc_pos, acc_d), (pos, d)| {
                if acc_d <= d {
                    (acc_pos, acc_d)
                } else {
                    (pos, d)
                }
            })
    }

    /// Finds the minimal distance of each column.
    #[allow(dead_code)]
    pub(crate) fn min_dist_cols(&self) -> [f32; RES_X] {
        let mut columns = [f32::MAX; RES_X];

        for (i, c) in self.zone_dist.iter().enumerate() {
            columns[i] = c
                .iter()
                .filter_map(|&d| if d > 0.0 { Some(d) } else { None })
                .fold(f32::MAX, f32::min)
        }

        columns
    }

    /// Attempts to recognize a hand from the measurement and finds its position, distance, etc. .
    pub(crate) fn recognize_hand(&self, params: &SensorParams, threshold_dist: f32) -> HandState {
        let hand_pos = self.hand_pos(params);

        if hand_pos.r > 0.0 && hand_pos.r <= threshold_dist {
            HandState::HandFound { hand_pos }
        } else {
            HandState::HandNotFound
        }
    }

    /// Attempts to find the hand position. Expects that there is at least one valid distance value in one of the zones,
    /// else returns invalid spherical coordinates.
    fn hand_pos(&self, params: &SensorParams) -> CoordsSpherical {
        // First calculate the positions of the zone measurements
        let mut zones_pos = [[CoordsSpherical::invalid(); RES_X]; RES_Y];

        for (pos_y, row) in self.zone_dist.iter().enumerate() {
            for (pos_x, zone) in row.iter().enumerate() {
                zones_pos[pos_y][pos_x] =
                    dist_position_spher::<RES_X, RES_Y>(*zone, pos_x, pos_y, params);
            }
        }

        let (min_dist_zone_pos, _min_dist) = self.min_dist();
        let min_pos_spher = zones_pos[min_dist_zone_pos[1]][min_dist_zone_pos[0]];

        // Then calculate the the hand position as weighted average mean,
        // given the smallest distance as initial position and weighing in the distance of the other measurements to it.

        // Calculates the weight of the positions in regards to min_pos for the weighted average mean
        fn pos_weight(min_pos: &CoordsCartesian, other: &CoordsCartesian) -> f32 {
            /// This is the factor that determines how much the distance weighs in into the average mean.
            ///
            /// E.g. a factor of 10.0 means that a position 1cm away from the measurement with min dist weighs in with
            /// value 10.0, a position 2cm away weighs in with value 5.0.
            const WEIGHT_FACTOR: f32 = 100.0;

            let dist_to = min_pos.dist_to(other);

            WEIGHT_FACTOR / dist_to
        }

        // The sum of the positions multiplied by the weight (numerator of the weighted mean)
        let coords_sum_weighted = (0..RES_Y)
            .flat_map(|pos_y| {
                (0..RES_X).filter_map(move |pos_x| {
                    if zones_pos[pos_y][pos_x].is_invalid() {
                        return None;
                    }
                    let c_cart = CoordsCartesian::from(zones_pos[pos_y][pos_x]);

                    let weight = if pos_x == min_dist_zone_pos[0] && pos_y == min_dist_zone_pos[1] {
                        // if this is the zone with min dist, skip the weight calc
                        1.0
                    } else {
                        pos_weight(&min_pos_spher.into(), &c_cart)
                    };

                    Some(CoordsCartesian {
                        x: c_cart.x * weight,
                        y: c_cart.y * weight,
                        z: c_cart.z * weight,
                    })
                })
            })
            .fold(CoordsCartesian::zero(), |mut acc, x| {
                acc.x += x.x;
                acc.y += x.y;
                acc.z += x.z;
                acc
            });

        // The sum of the weights (denominator of the weighted mean)
        let summed_weight: f32 = (0..RES_Y)
            .flat_map(|pos_y| {
                (0..RES_X).filter_map(move |pos_x| {
                    if zones_pos[pos_y][pos_x].is_invalid() {
                        return None;
                    }
                    let c_cart = CoordsCartesian::from(zones_pos[pos_y][pos_x]);

                    let weight = if pos_x == min_dist_zone_pos[0] && pos_y == min_dist_zone_pos[1] {
                        // if this is the zone with min dist, skip the weight calc
                        1.0
                    } else {
                        pos_weight(&min_pos_spher.into(), &c_cart)
                    };

                    Some(weight)
                })
            })
            .sum();

        if summed_weight == 0.0 {
            return CoordsSpherical::invalid();
        }

        let mean = CoordsCartesian {
            x: coords_sum_weighted.x / summed_weight,
            y: coords_sum_weighted.y / summed_weight,
            z: coords_sum_weighted.z / summed_weight,
        };
        mean.into()

        /*
        // Simply use the zone with min dist as pos
                let (min_dist_zone_pos, min_dist) = self.min_dist();

                dist_position_spher::<RES_X, RES_Y>(
                    min_dist,
                    min_dist_zone_pos[0],
                    min_dist_zone_pos[1],
                    params,
                )
        */
    }
}

/// Calculates the position in space for the measurement in a sensor zone.
///
/// Arguments:
/// - dist: the distance value of the measurement
/// - zone_pos_x: The x-index of the zone in the sensor grid
/// - zone_pos_y: The y-index of the zone in the sensor grid
///
/// Returns the position in spherical coordinates.
pub(crate) fn dist_position_spher<const RES_X: usize, const RES_Y: usize>(
    dist: f32,
    zone_pos_x: usize,
    zone_pos_y: usize,
    params: &SensorParams,
) -> CoordsSpherical {
    let angle_per_zone_hor = (params.fov_horizontal / (RES_X as f32)).to_radians();
    let angle_per_zone_vert = (params.fov_vertical / (RES_Y as f32)).to_radians();

    let r = dist;

    if r < 0.0 {
        return CoordsSpherical::invalid();
    }

    let theta = (zone_pos_x as f32 - (RES_X as f32 / 2.0)) * angle_per_zone_hor;
    // 90.0 deg - .. because the z-axis is pointing up from the center of the sensor grid
    let phi = FRAC_PI_2 - (zone_pos_y as f32 - (RES_Y as f32 / 2.0)) * angle_per_zone_vert;

    CoordsSpherical { r, theta, phi }
}

/// Finds the nearest zone for all given measurements.
///
/// Returns the tuple:
///
/// `("index of the measurement containing the nearest zone", ["x-pos in matrix", "y-pos in matrix"], "distance")`
pub(crate) fn find_nearest_zone<
    const RES_X: usize,
    const RES_Y: usize,
    T: IntoIterator<Item = SensorMeasurement<RES_X, RES_Y>>,
>(
    measurements: T,
) -> (usize, [usize; 2], f32) {
    measurements
        .into_iter()
        .enumerate()
        .fold((0, [0, 0], f32::MAX), |acc, m| {
            let m_min_dist = m.1.min_dist();

            if acc.2 <= m_min_dist.1 {
                acc
            } else {
                (m.0, m_min_dist.0, m_min_dist.1)
            }
        })
}

/// Finds the nearest hand position for all given hand states.
///
/// If the iterator contains no valid hand state, it returns index 0 and invalid spherical coordinates.
///
/// Returns the tuple:
///
/// `("index of the hand state containing the nearest pos", "hand pos")`
#[allow(dead_code)]
pub(crate) fn find_nearest_hand_pos<T: IntoIterator<Item = HandState>>(
    states: T,
) -> (usize, CoordsSpherical) {
    states
        .into_iter()
        .enumerate()
        .filter_map(|(i, hs)| {
            if let HandState::HandFound { hand_pos } = hs {
                Some((i, hand_pos))
            } else {
                None
            }
        })
        .fold(
            (0, CoordsSpherical::invalid()),
            |(acc_i, acc_pos), (i, pos)| {
                if pos.r < acc_pos.r {
                    (i, pos)
                } else {
                    (acc_i, acc_pos)
                }
            },
        )
}
