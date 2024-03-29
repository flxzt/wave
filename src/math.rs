//! Math Utilities.

use core::iter::Sum;
use core::ops::{Add, Div, Sub};

/// Cartesian coordinates.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoordsCartesian {
    /// The distance to the origin on the x-axis.
    pub x: f32,
    /// The distance to the origin on the y-axis.
    pub y: f32,
    /// The distance to the origin on the z-axis.
    pub z: f32,
}

impl From<CoordsSpherical> for CoordsCartesian {
    fn from(spher: CoordsSpherical) -> Self {
        Self {
            x: spher.r * libm::cosf(spher.theta) * libm::sinf(spher.phi),
            y: spher.r * libm::sinf(spher.theta) * libm::sinf(spher.phi),
            z: spher.r * libm::cosf(spher.phi),
        }
    }
}

impl CoordsCartesian {
    /// The origin.
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculates the euclidean distance to other.
    pub fn dist_to(&self, other: &Self) -> f32 {
        libm::sqrtf(
            libm::powf(self.x - other.x, 2.0)
                + libm::powf(self.y - other.y, 2.0)
                + libm::powf(self.z - other.z, 2.0),
        )
    }
}

/// Represents spherical coordinates in mathematical naming convention.
/// ([Reference](https://mathworld.wolfram.com/SphericalCoordinates.html))
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoordsSpherical {
    /// Distance to the origin.
    pub r: f32,
    /// Angle with respect to x-axis (azimuth) (rad).
    pub theta: f32,
    /// Angle with respect to polar / z-axis (zenith) (rad).
    pub phi: f32,
}

impl From<CoordsCartesian> for CoordsSpherical {
    fn from(cart: CoordsCartesian) -> Self {
        let r = libm::sqrtf(
            libm::powf(cart.x, 2.0) + libm::powf(cart.y, 2.0) + libm::powf(cart.z, 2.0),
        );
        let theta = libm::atan2f(cart.y, cart.x);
        let phi = libm::acosf(cart.z / r);

        Self { r, theta, phi }
    }
}

impl CoordsSpherical {
    /// Returns invalid coordinates (`r` is set to `-1.0`).
    pub fn invalid() -> Self {
        Self {
            r: -1.0,
            theta: 0.0,
            phi: 0.0,
        }
    }

    /// Checks if the coorindate is invalid (`r` is < `0.0`).
    pub fn is_invalid(&self) -> bool {
        self.r < 0.0
    }

    /// Origin coordinates (`r`, `theta` and `phi` are set to `0.0`).
    pub fn zero() -> Self {
        Self {
            r: 0.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

/// Transposes the given matrix.
pub fn matrix_2d_transpose<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; ROWS]; COLS]
where
    T: Copy + Default,
{
    let mut transposed = [[T::default(); ROWS]; COLS];

    for (j, row) in matrix.into_iter().enumerate() {
        for (i, value) in row.into_iter().enumerate() {
            transposed[i][j] = value;
        }
    }

    transposed
}

/// Rotates the matrix 90 degrees in clockwise direction.
pub fn matrix_2d_rotate_90deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; ROWS]; COLS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_rows(matrix_2d_transpose(matrix))
}

/// Rotates the matrix 180 degrees.
pub fn matrix_2d_rotate_180deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; COLS]; ROWS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_rows(matrix_2d_reverse_cols(matrix))
}

/// Rotates the matrix 270 degrees in clockwise direction.
pub fn matrix_2d_rotate_270deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; ROWS]; COLS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_cols(matrix_2d_transpose(matrix))
}

/// Mirror / reverse the columns of the matrix.
pub fn matrix_2d_reverse_cols<const ROWS: usize, const COLS: usize, T>(
    mut matrix: [[T; COLS]; ROWS],
) -> [[T; COLS]; ROWS]
where
    T: Copy + Default,
{
    matrix.reverse();

    matrix
}

/// Mirror / reverse the rows of the matrix.
pub fn matrix_2d_reverse_rows<const ROWS: usize, const COLS: usize, T>(
    mut matrix: [[T; COLS]; ROWS],
) -> [[T; COLS]; ROWS]
where
    T: Copy + Default,
{
    for row in matrix.iter_mut() {
        row.reverse();
    }

    matrix
}

/// Iterator adapter that calculates the moving average.
#[derive(Debug, Clone)]
pub struct MovingAvg<I, T>
where
    I: Clone,
    T: Clone,
{
    iter: I,
    window_size: u32,
    moving_sum: Option<T>,
}

impl<I, T> Iterator for MovingAvg<I, T>
where
    I: ExactSizeIterator<Item = T> + Clone,
    T: Clone + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Sum + From<u32>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(moving_sum) = self.moving_sum.as_mut() {
            let window_next = self.iter.clone().nth(self.window_size as usize)?;
            let window_prev = self.iter.next()?;

            *moving_sum = moving_sum.clone() - window_prev + window_next;
        } else {
            self.moving_sum = Some(self.iter.clone().take(self.window_size as usize).sum());
        }
        Some(self.moving_sum.clone().unwrap() / self.window_size.into())
    }
}

impl<I, T> MovingAvg<I, T>
where
    I: Clone,
    T: Clone,
{
    /// A new moving average iterator adapter.
    pub fn new(iter: I, window_size: u32) -> Self {
        Self {
            iter,
            window_size,
            moving_sum: None,
        }
    }
}

/// The iterator trait for [MovingAvg].
pub trait MovingAvgIter<T>: Iterator<Item = T> + Sized
where
    Self: Clone,
    T: Clone,
{
    /// Calculate the moving average with the given window size.
    fn moving_avg(self, window_size: u32) -> MovingAvg<Self, T> {
        MovingAvg::new(self, window_size)
    }
}

impl<I, T> MovingAvgIter<T> for I
where
    I: Iterator<Item = T> + Clone,
    T: Clone,
{
}

#[cfg(test)]
mod tests {
    use super::{CoordsCartesian, CoordsSpherical, MovingAvgIter};
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn coords_cart_to_spher() {
        let coords_cart = CoordsCartesian {
            x: 10.0,
            y: 1.0,
            z: 2.0,
        };

        let coords_spher = CoordsSpherical::from(coords_cart);

        assert_relative_eq!(coords_spher.r, 10.24695076596);
        assert_relative_eq!(coords_spher.theta, 5.7105931374996_f32.to_radians());
        assert_relative_eq!(coords_spher.phi, 78.744760267338_f32.to_radians());
    }

    #[test]
    fn coords_spher_to_cart() {
        let coords_spher = CoordsSpherical {
            r: 10.24695076596,
            theta: 5.7105931374996_f32.to_radians(),
            phi: 78.744760267338_f32.to_radians(),
        };

        let coords_cart = CoordsCartesian::from(coords_spher);

        assert_relative_eq!(coords_cart.x, 10.0);
        assert_relative_eq!(coords_cart.y, 1.0);
        assert_relative_eq!(coords_cart.z, 2.0);
    }

    #[test]
    fn matrix_2d_transpose() {
        let m = [
            [1.0, 2.0, 3.0],
            [1.5, 2.5, 3.5],
            [2.0, 3.0, 4.0],
            [2.5, 3.5, 4.5],
        ];

        assert_eq!(
            super::matrix_2d_transpose(m),
            [
                [1.0, 1.5, 2.0, 2.5],
                [2.0, 2.5, 3.0, 3.5],
                [3.0, 3.5, 4.0, 4.5],
            ]
        );
    }

    #[test]
    fn matrix_2d_rotate_90deg() {
        let m = [
            [1.0, 2.0, 3.0],
            [1.5, 2.5, 3.5],
            [2.0, 3.0, 4.0],
            [2.5, 3.5, 4.5],
        ];

        assert_eq!(
            super::matrix_2d_rotate_90deg(m),
            [
                [2.5, 2.0, 1.5, 1.0],
                [3.5, 3.0, 2.5, 2.0],
                [4.5, 4.0, 3.5, 3.0],
            ]
        );
    }

    #[test]
    fn matrix_2d_rotate_180deg() {
        let m = [
            [1.0, 2.0, 3.0],
            [1.5, 2.5, 3.5],
            [2.0, 3.0, 4.0],
            [2.5, 3.5, 4.5],
        ];

        assert_eq!(
            super::matrix_2d_rotate_180deg(m),
            [
                [4.5, 3.5, 2.5],
                [4.0, 3.0, 2.0],
                [3.5, 2.5, 1.5],
                [3.0, 2.0, 1.0],
            ]
        );
    }

    #[test]
    fn matrix_2d_rotate_270deg() {
        let m = [
            [1.0, 2.0, 3.0],
            [1.5, 2.5, 3.5],
            [2.0, 3.0, 4.0],
            [2.5, 3.5, 4.5],
        ];

        assert_eq!(
            super::matrix_2d_rotate_270deg(m),
            [
                [3.0, 3.5, 4.0, 4.5],
                [2.0, 2.5, 3.0, 3.5],
                [1.0, 1.5, 2.0, 2.5]
            ]
        );
    }

    #[test]
    fn matrix_2d_reverse_cols() {
        let m = [
            [1.0, 2.0, 3.0, 4.0],
            [1.5, 2.5, 3.5, 4.5],
            [2.0, 3.0, 4.0, 5.0],
            [2.5, 3.5, 4.5, 5.5],
        ];

        assert_eq!(
            super::matrix_2d_reverse_cols(m),
            [
                [2.5, 3.5, 4.5, 5.5],
                [2.0, 3.0, 4.0, 5.0],
                [1.5, 2.5, 3.5, 4.5],
                [1.0, 2.0, 3.0, 4.0],
            ]
        );
    }

    #[test]
    fn matrix_2d_reverse_rows() {
        let m = [
            [1.0, 2.0, 3.0, 4.0],
            [1.5, 2.5, 3.5, 4.5],
            [2.0, 3.0, 4.0, 5.0],
            [2.5, 3.5, 4.5, 5.5],
        ];

        assert_eq!(
            super::matrix_2d_reverse_rows(m),
            [
                [4.0, 3.0, 2.0, 1.0],
                [4.5, 3.5, 2.5, 1.5],
                [5.0, 4.0, 3.0, 2.0],
                [5.5, 4.5, 3.5, 2.5],
            ]
        );
    }

    #[test]
    fn moving_avg() {
        const WINDOW_SIZE: usize = 3;
        let samples: [f64; 8] = [5.0, 2.2, 3.8, 8.0, 4.1, 1.0, 2.5, 3.0];
        let expected: [f64; 8 - WINDOW_SIZE + 1] = [
            11. / 3.,
            14. / 3.,
            53. / 10.,
            131. / 30.,
            38. / 15.,
            13. / 6.,
        ];
        for (i, res) in samples.into_iter().moving_avg(3).enumerate() {
            assert_relative_eq!(expected[i], res, epsilon = 0.0001);
        }
    }
}
