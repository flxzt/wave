//! helper math functions to prepare measurements, convert coordinates, etc.

/// Cartesian coordinates
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoordsCartesian {
    /// The distance to the origin on the x-axis
    pub x: f32,
    /// The distance to the origin on the z-axis
    pub y: f32,
    /// The distance to the origin on the y-axis
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
    /// The origin
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculates the eucledian distance to other coordinates
    pub fn dist_to(&self, other: &Self) -> f32 {
        libm::sqrtf(
            libm::powf(self.x - other.x, 2.0)
                + libm::powf(self.y - other.y, 2.0)
                + libm::powf(self.z - other.z, 2.0),
        )
    }
}

/// Represents spherical coordinates in mathematical naming convention (https://mathworld.wolfram.com/SphericalCoordinates.html)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoordsSpherical {
    /// distance to the origin
    pub r: f32,
    /// angle with respect to x-axis (azimuth) (rad)
    pub theta: f32,
    /// angle with respect to polar axis / z-axis (zenith) (rad)
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
    /// Returns invalid coorindates ( r is set to -1.0 )
    pub fn invalid() -> Self {
        Self {
            r: -1.0,
            theta: 0.0,
            phi: 0.0,
        }
    }

    /// Checks if the coorindate is invalid ( r is -1.0 )
    pub fn is_invalid(&self) -> bool {
        self.r < 0.0
    }

    /// Returns the origin coordinates (r, theta and phi are set to 0.0 )
    pub fn zero() -> Self {
        Self {
            r: 0.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

/// Transposes the given matrix
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

/// Rotates the matrix 90 degrees in clockwise direction
pub fn matrix_2d_rotate_90deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; ROWS]; COLS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_rows(matrix_2d_transpose(matrix))
}

/// Rotates the matrix 180 degrees
pub fn matrix_2d_rotate_180deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; COLS]; ROWS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_rows(matrix_2d_reverse_cols(matrix))
}

/// Rotates the matrix 270 degrees in clockwise direction
pub fn matrix_2d_rotate_270deg<const ROWS: usize, const COLS: usize, T>(
    matrix: [[T; COLS]; ROWS],
) -> [[T; ROWS]; COLS]
where
    T: Copy + Default,
{
    matrix_2d_reverse_cols(matrix_2d_transpose(matrix))
}

/// Mirror / reverse the columns of the matrix
pub fn matrix_2d_reverse_cols<const ROWS: usize, const COLS: usize, T>(
    mut matrix: [[T; COLS]; ROWS],
) -> [[T; COLS]; ROWS]
where
    T: Copy + Default,
{
    matrix.reverse();

    matrix
}

/// Mirror / reverse the rows of the matrix
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use pretty_assertions::assert_eq;

    use super::{CoordsCartesian, CoordsSpherical};

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
}
