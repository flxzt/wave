//! C Bindings.

use crate::math::{CoordsCartesian, CoordsSpherical};
use crate::measurements::SensorParams;
use crate::{
    GestureRecognizer, RecognizerParams, RecognizerResult, RecognizerStatus, SensorMeasurement,
};

/// The horizontal sensor resolution.
///
/// For users of the C bindings:
/// Change the value of this definition to change the x-axis resolution of the sensor.
///
/// For VL53L5CX: Resolution either 4x4 or 8x8.
pub const RES_X: usize = 8;

/// The vertical sensor resolution.
///
/// For users of the C bindings:
/// Change the value of this definition to change the y-axis resolution of the sensor.
///
/// For VL53L5CX: Resolution either 4x4 or 8x8.
pub const RES_Y: usize = 8;

/// The history size.
///
/// Must be large enough to hold at least ca 2 seconds of data to be able to reliably recognize gestures.
pub const HISTORY_SIZE: usize = 60;

/// Creates an invalid measurement (distances are set to `-1.0`).
#[no_mangle]
pub extern "C" fn sensor_measurement_invalid() -> SensorMeasurement<RES_X, RES_Y> {
    SensorMeasurement::<RES_X, RES_Y>::invalid()
}

/// Default sensor parameters for the ST VL53L5CX TOF-Sensor.
#[no_mangle]
pub extern "C" fn sensor_params_default_vl53l5cx() -> SensorParams {
    SensorParams::default_vl53l5cx()
}

/// Default recognizer parameters, providing a good starting point for gesture recognition.
#[no_mangle]
pub extern "C" fn recognizer_params_default() -> RecognizerParams {
    RecognizerParams::default()
}

/// Default result, meaning no hand is found and no gestures are recognized.
///
/// Used to initialize the result before passing it to the recognizer.
#[no_mangle]
pub extern "C" fn recognizer_result_default() -> RecognizerResult {
    RecognizerResult::default()
}

/// A new gesture recognizer with the given parameters.
///
/// The sensor parameters have preconfigured defaults for common TOF-Sensors such as the ST VL53L5CX.
#[no_mangle]
pub extern "C" fn gesture_recognizer_new(
    params: RecognizerParams,
    sensor_params: SensorParams,
) -> GestureRecognizer<RES_X, RES_Y, HISTORY_SIZE> {
    GestureRecognizer::new(params, sensor_params)
}

/// Resets the gesture recognizer with the given parameters.
///
/// Clears the history, ongoing predictions and resets the internal state.
#[no_mangle]
pub extern "C" fn gesture_recognizer_reset(
    gesture_recognizer: &mut GestureRecognizer<RES_X, RES_Y, HISTORY_SIZE>,
    params: RecognizerParams,
    sensor_params: SensorParams,
    now: u32,
) -> RecognizerStatus {
    gesture_recognizer.reset(params, sensor_params, now)
}

/// Updates the gesture recognizer with a new measurement and time.
///
/// The time in the measurement must be monotonically increasing (usually coming from a systick timer).
#[no_mangle]
pub extern "C" fn gesture_recognizer_update(
    gesture_recognizer: &mut GestureRecognizer<RES_X, RES_Y, HISTORY_SIZE>,
    measurement: SensorMeasurement<RES_X, RES_Y>,
    gesture_result: &mut RecognizerResult,
) -> RecognizerStatus {
    gesture_recognizer.update(measurement, gesture_result)
}

/// Converts cartesian to spherical coordinates.
#[no_mangle]
pub extern "C" fn coords_spherical_from_cartesian(coords_cart: CoordsCartesian) -> CoordsSpherical {
    coords_cart.into()
}

/// Converts spherical to cartesian coordinates.
#[no_mangle]
pub extern "C" fn coords_cartesian_from_spherical(
    coords_spher: CoordsSpherical,
) -> CoordsCartesian {
    coords_spher.into()
}
