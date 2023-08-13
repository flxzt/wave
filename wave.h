#ifndef WAVE_H
#define WAVE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * The horizontal sensor resolution.
 *
 * For users of the C bindings:
 * Change the value of this definition to change the x-axis resolution of the sensor.
 *
 * For VL53L5CX: Resolution either 4x4 or 8x8.
 */
#define RES_X 8

/**
 * The vertical sensor resolution.
 *
 * For users of the C bindings:
 * Change the value of this definition to change the y-axis resolution of the sensor.
 *
 * For VL53L5CX: Resolution either 4x4 or 8x8.
 */
#define RES_Y 8

/**
 * The history size.
 *
 * Must be large enough to hold at least ca 2 seconds of data to be able to reliably recognize gestures.
 */
#define HISTORY_SIZE 60

/**
 * A hand gesture.
 */
typedef enum Gesture {
  /**
   * No recognized gesture.
   */
  GestureNone = 0,
  /**
   * A static hold.
   */
  GestureStaticHold,
  /**
   * A right swipe.
   */
  GestureSwipeRight,
  /**
   * A left swipe.
   */
  GestureSwipeLeft,
  /**
   * A up swipe.
   */
  GestureSwipeUp,
  /**
   * A down swipe.
   */
  GestureSwipeDown,
} Gesture;

/**
 * The status of the gesture recognizer.
 */
typedef enum RecognizerStatus {
  /**
   * Ok.
   */
  RecognizerStatusOk = 0,
  /**
   * Indicates a failure while the recognizer was initialized.
   */
  RecognizerStatusInitFailure,
  /**
   * Indicates invalid input to the recognizer.
   */
  RecognizerStatusInvalidInput,
} RecognizerStatus;

/**
 * Represents a sensor measurement coming from the TOF sensor.
 *
 * Expects that the zones are already rotated and mirrored,
 * so that the zone with index \[0\]\[0\] is the top left corner when looking at the sensor.
 */
typedef struct SensorMeasurement_RES_X__RES_Y {
  /**
   * The measured distances of each zone.
   *
   * Invalid distance measurements are represented by value -1.0.
   */
  float zone_dist[RES_Y][RES_X];
  /**
   * The time of the measurement in milliseconds. Must be monotonically increasing.
   */
  uint32_t time_ms;
} SensorMeasurement_RES_X__RES_Y;

/**
 * Configurable sensor parameters. Different for every sensor.
 */
typedef struct SensorParams {
  /**
   * The horizontal FOV of the sensor.
   */
  float fov_horizontal;
  /**
   * The vertical FOV of the sensor.
   */
  float fov_vertical;
} SensorParams;

/**
 * Parameters for gesture recognition.
 */
typedef struct RecognizerParams {
  /**
   * The furthest hand distance for gesture recognition.
   */
  float gesture_threshold_dist;
  /**
   * The time the hand has to be still to recognize a static hold.
   */
  uint32_t static_hold_time_ms;
  /**
   * How much the hand can move towards / away from the sensor while doing a static hold.
   */
  float static_hold_tolerance_dist;
  /**
   * How much the hand can move towards / away from the sensor while doing a swipe.
   */
  float swipe_tolerance_dist;
  /**
   * How much distance the hand has to travel to detect a horizontal swipe.
   */
  float swipe_horizontal_travel_dist;
  /**
   * How much distance the hand has to travel to detect a vertical swipe.
   */
  float swipe_vertical_travel_dist;
} RecognizerParams;

/**
 * Represents spherical coordinates in mathematical naming convention.
 * ([Reference](https://mathworld.wolfram.com/SphericalCoordinates.html))
 */
typedef struct CoordsSpherical {
  /**
   * Distance to the origin.
   */
  float r;
  /**
   * Angle with respect to x-axis (azimuth) (rad).
   */
  float theta;
  /**
   * Angle with respect to polar / z-axis (zenith) (rad).
   */
  float phi;
} CoordsSpherical;

/**
 * The recognized hand state.
 */
typedef enum HandState_Tag {
  /**
   * No hand was found.
   */
  HandNotFound,
  /**
   * Hand was found with this position.
   */
  HandFound,
} HandState_Tag;

typedef struct HandFound_Body {
  /**
   * The hand position in spherical coordinates.
   */
  struct CoordsSpherical hand_pos;
} HandFound_Body;

typedef struct HandState {
  HandState_Tag tag;
  union {
    HandFound_Body hand_found;
  };
} HandState;

/**
 * A gesture prediction result.
 */
typedef struct RecognizerResult {
  /**
   * The current hand state.
   */
  struct HandState hand_state;
  /**
   * The recognized gesture, GestureNone if no gesture was recognized.
   */
  enum Gesture gesture;
} RecognizerResult;

typedef struct HistoryEntry_RES_X__RES_Y {
  struct SensorMeasurement_RES_X__RES_Y measurement;
  struct HandState hand_state;
} HistoryEntry_RES_X__RES_Y;

/**
 * The gesture recognizer.
 *
 * Is initially configured through parameters and gets fed measurements and time and predicts gestures.
 */
typedef struct GestureRecognizer_RES_X__RES_Y__HISTORY_SIZE {
  struct RecognizerParams params;
  struct SensorParams sensor_params;
  uint32_t start_time;
  struct HistoryEntry_RES_X__RES_Y history[HISTORY_SIZE];
  uintptr_t received_measurements;
} GestureRecognizer_RES_X__RES_Y__HISTORY_SIZE;

/**
 * Cartesian coordinates.
 */
typedef struct CoordsCartesian {
  /**
   * The distance to the origin on the x-axis.
   */
  float x;
  /**
   * The distance to the origin on the y-axis.
   */
  float y;
  /**
   * The distance to the origin on the z-axis.
   */
  float z;
} CoordsCartesian;

/**
 * Creates an invalid measurement (distances are set to `-1.0`).
 */
struct SensorMeasurement_RES_X__RES_Y sensor_measurement_invalid(void);

/**
 * Default sensor parameters for the ST VL53L5CX TOF-Sensor.
 */
struct SensorParams sensor_params_default_vl53l5cx(void);

/**
 * Default recognizer parameters, providing a good starting point for gesture recognition.
 */
struct RecognizerParams recognizer_params_default(void);

/**
 * Default result, meaning no hand is found and no gestures are recognized.
 *
 * Used to initialize the result before passing it to the recognizer.
 */
struct RecognizerResult recognizer_result_default(void);

/**
 * A new gesture recognizer with the given parameters.
 *
 * The sensor parameters have preconfigured defaults for common TOF-Sensors such as the ST VL53L5CX.
 */
struct GestureRecognizer_RES_X__RES_Y__HISTORY_SIZE gesture_recognizer_new(struct RecognizerParams params,
                                                                           struct SensorParams sensor_params);

/**
 * Resets the gesture recognizer with the given parameters.
 *
 * Clears the history, ongoing predictions and resets the internal state.
 */
enum RecognizerStatus gesture_recognizer_reset(struct GestureRecognizer_RES_X__RES_Y__HISTORY_SIZE *gesture_recognizer,
                                               struct RecognizerParams params,
                                               struct SensorParams sensor_params,
                                               uint32_t now);

/**
 * Updates the gesture recognizer with a new measurement and time.
 *
 * The time in the measurement must be monotonically increasing (usually coming from a systick timer).
 */
enum RecognizerStatus gesture_recognizer_update(struct GestureRecognizer_RES_X__RES_Y__HISTORY_SIZE *gesture_recognizer,
                                                struct SensorMeasurement_RES_X__RES_Y measurement,
                                                struct RecognizerResult *gesture_result);

/**
 * Converts cartesian to spherical coordinates.
 */
struct CoordsSpherical coords_spherical_from_cartesian(struct CoordsCartesian coords_cart);

/**
 * Converts spherical to cartesian coordinates.
 */
struct CoordsCartesian coords_cartesian_from_spherical(struct CoordsSpherical coords_spher);

#endif /* WAVE_H */
