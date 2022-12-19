//! Gesture Recognizer
//!
//! All distance values are considered to be in millimeter.

use crate::math::CoordsCartesian;
use crate::measurements::SensorParams;
use crate::{measurements, HandState, SensorMeasurement};

/// Represents a hand gesture
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Gesture {
    /// No recognized gesture
    GestureNone = 0,
    /// A static hold
    GestureStaticHold,
    /// A right swipe
    GestureSwipeRight,
    /// A left swipe
    GestureSwipeLeft,
    /// A up swipe
    GestureSwipeUp,
    /// A down swipe
    GestureSwipeDown,
}

/// A gesture prediction result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RecognizerResult {
    /// The current hand state
    pub hand_state: HandState,
    /// The recognized gesture, GestureNone if no gesture was recognized
    pub gesture: Gesture,
}

impl Default for RecognizerResult {
    fn default() -> Self {
        Self {
            hand_state: HandState::HandNotFound,
            gesture: Gesture::GestureNone,
        }
    }
}

/// Parameters for gesture recognition
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RecognizerParams {
    /// The furthest hand distance for gesture recognition
    pub gesture_threshold_dist: f32,
    /// The time the hand has to be still to recognize a static hold
    pub static_hold_time_ms: u32,
    /// How much the hand can move towards / away from the sensor while doing a static hold
    pub static_hold_tolerance_dist: f32,
    /// How much the hand can move towards / away from the sensor while doing a swipe
    pub swipe_tolerance_dist: f32,
    /// How much distance the hand has to travel to detect a horizontal swipe
    pub swipe_horizontal_travel_dist: f32,
    /// How much distance the hand has to travel to detect a vertical swipe
    pub swipe_vertical_travel_dist: f32,
}

impl Default for RecognizerParams {
    fn default() -> Self {
        Self {
            gesture_threshold_dist: 400.0,
            static_hold_time_ms: 1500,
            static_hold_tolerance_dist: 100.0,
            swipe_tolerance_dist: 120.0,
            swipe_horizontal_travel_dist: 80.0,
            swipe_vertical_travel_dist: 70.0,
        }
    }
}

/// The status of the gesture recognizer
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum RecognizerStatus {
    /// Ok
    RecognizerStatusOk = 0,
    /// Indicates a failure while the recognizer was initialized
    RecognizerStatusInitFailure,
    /// Indicates invalid input to the recognizer
    RecognizerStatusInvalidInput,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct HistoryEntry<const RES_X: usize, const RES_Y: usize> {
    measurement: SensorMeasurement<RES_X, RES_Y>,
    hand_state: HandState,
}

impl<const RES_X: usize, const RES_Y: usize> HistoryEntry<RES_X, RES_Y> {
    pub fn invalid() -> Self {
        Self {
            measurement: SensorMeasurement::invalid(),
            hand_state: HandState::HandNotFound,
        }
    }
}

pub(crate) fn iter_history_newer<
    const RES_X: usize,
    const RES_Y: usize,
    T: IntoIterator<Item = HistoryEntry<RES_X, RES_Y>>,
>(
    entries: T,
    newer_than_ms: u32,
    now: u32,
) -> impl Iterator<Item = HistoryEntry<RES_X, RES_Y>> {
    entries
        .into_iter()
        .filter(move |e| now - e.measurement.time_ms < newer_than_ms)
}

pub(crate) fn iter_history_older_eq<
    const RES_X: usize,
    const RES_Y: usize,
    T: IntoIterator<Item = HistoryEntry<RES_X, RES_Y>>,
>(
    entries: T,
    older_eq_ms: u32,
    now: u32,
) -> impl Iterator<Item = HistoryEntry<RES_X, RES_Y>> {
    entries
        .into_iter()
        .filter(move |e| now - e.measurement.time_ms >= older_eq_ms)
}

/// The gesture recognizer.
///
/// Is initially configured through parameters and gets fed measurements and time and predicts gestures.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GestureRecognizer<const RES_X: usize, const RES_Y: usize, const HISTORY_SIZE: usize> {
    params: RecognizerParams,
    sensor_params: SensorParams,
    start_time: u32,
    history: [HistoryEntry<RES_X, RES_Y>; HISTORY_SIZE],
    received_measurements: usize,
}

impl<const RES_X: usize, const RES_Y: usize, const HISTORY_SIZE: usize>
    GestureRecognizer<RES_X, RES_Y, HISTORY_SIZE>
{
    /// A new gesture recognizer with the given parameters.
    ///
    /// The sensor parameters have preconfigured defaults for common TOF-Sensors such as the ST VL53L5CX.
    pub fn new(params: RecognizerParams, sensor_params: SensorParams) -> Self {
        Self {
            params,
            sensor_params,
            start_time: 0,
            history: [HistoryEntry::invalid(); HISTORY_SIZE],
            received_measurements: 0,
        }
    }

    /// Resets the gesture recognizer with the given parameters.
    ///
    /// Clears the history, ongoing predictions and resets the internal state.
    pub fn reset(
        &mut self,
        params: RecognizerParams,
        sensor_params: SensorParams,
        now: u32,
    ) -> RecognizerStatus {
        self.params = params;
        self.sensor_params = sensor_params;
        self.start_time = now;
        self.clear_history();

        RecognizerStatus::RecognizerStatusOk
    }

    /// Updates the gesture recognizer with a new measurement and time.
    ///
    /// The time in the measurement must be monotonically increasing (usually coming from a systick timer).
    pub fn update(
        &mut self,
        measurement: SensorMeasurement<RES_X, RES_Y>,
        recognizer_result: &mut RecognizerResult,
    ) -> RecognizerStatus {
        *recognizer_result = RecognizerResult::default();
        let now = measurement.time_ms;

        // Time must be monotonically increasing
        if measurement.time_ms <= self.history.last().unwrap().measurement.time_ms {
            return RecognizerStatus::RecognizerStatusInvalidInput;
        }

        let hand_state =
            measurement.recognize_hand(&self.sensor_params, self.params.gesture_threshold_dist);

        self.push_to_history(HistoryEntry {
            measurement,
            hand_state,
        });

        recognizer_result.hand_state = hand_state;

        recognizer_result.gesture = self.recognize_gesture(now);

        RecognizerStatus::RecognizerStatusOk
    }

    /// Gets the current configured sensor parameters
    pub fn get_sensor_params(&self) -> SensorParams {
        self.sensor_params
    }

    /// Gets the current configured gesture recognizer parameters
    pub fn get_params(&self) -> RecognizerParams {
        self.params
    }

    /// Pushes an entry to the history
    fn push_to_history(&mut self, entry: HistoryEntry<RES_X, RES_Y>) {
        self.history.rotate_right(1);
        self.history[0] = entry;
        self.received_measurements += 1;
    }

    /// Clears the history and fills it with invalid measurements and state (all dist values set to -1.0)
    fn clear_history(&mut self) {
        for entry in self.history.iter_mut() {
            *entry = HistoryEntry::invalid();
        }

        self.received_measurements = 0;
    }

    /// Attempts to recognize a gesture from the measurements
    fn recognize_gesture(&mut self, now: u32) -> Gesture {
        let mut gesture = Gesture::GestureNone;

        if self.find_static_hold(now) {
            gesture = Gesture::GestureStaticHold;

            self.clear_history();
            return gesture;
        }

        let swipe_gesture = self.find_swipe(now);
        if swipe_gesture != Gesture::GestureNone {
            gesture = swipe_gesture;

            self.clear_history();
            return gesture;
        }

        gesture
    }

    /// Attempts to recognize a static hold.
    ///
    /// Returns true when a hold is recognized, else false.
    fn find_static_hold(&self, now: u32) -> bool {
        if self.received_measurements < HISTORY_SIZE.min(15) {
            return false;
        }

        let abs_min = measurements::find_nearest_zone(
            iter_history_newer(self.history, self.params.static_hold_time_ms, now)
                .map(|e| e.measurement),
        );

        if abs_min.2 <= 0.0 || abs_min.2 > self.params.gesture_threshold_dist {
            return false;
        }

        // Only returns true if all measurements meet the condition.
        !iter_history_newer(self.history, self.params.static_hold_time_ms, now).any(|e| {
            let zone_dist = e.measurement.zone_dist[abs_min.1[0]][abs_min.1[1]];

            if zone_dist <= 0.0 || zone_dist > self.params.gesture_threshold_dist {
                return true;
            }

            zone_dist > abs_min.2 + self.params.static_hold_tolerance_dist
                || zone_dist < abs_min.2 - self.params.static_hold_tolerance_dist
        })
    }

    /// Tries to recognize a swipe gesture.
    ///
    /// Returns either GestureSwipeRight / GestureSwipeLeft or GestureNone if no swipe was found.
    fn find_swipe(&self, now: u32) -> Gesture {
        if self.received_measurements < HISTORY_SIZE.min(15) {
            return Gesture::GestureNone;
        }

        for e in iter_history_newer(iter_history_older_eq(self.history, 300, now), 600, now) {
            if let HandState::HandFound { hand_pos } = e.hand_state {
                for n in iter_history_newer(self.history, 300, now) {
                    // Preconditions for a detected swipe in any direction
                    if let HandState::HandFound {
                        hand_pos: hand_pos_newer,
                    } = n.hand_state
                    {
                        let hand_pos_cart = CoordsCartesian::from(hand_pos);
                        let hand_pos_newer_cart = CoordsCartesian::from(hand_pos_newer);

                        if (hand_pos_newer_cart.x
                            >= hand_pos_cart.x - self.params.swipe_tolerance_dist)
                            && (hand_pos_newer_cart.x
                                < hand_pos_cart.x + self.params.swipe_tolerance_dist)
                        {
                            // Detect right swipe
                            if hand_pos_newer_cart.y - hand_pos_cart.y
                                > self.params.swipe_horizontal_travel_dist
                            {
                                return Gesture::GestureSwipeRight;
                            }

                            // Detect left swipe
                            if hand_pos_newer_cart.y - hand_pos_cart.y
                                < -self.params.swipe_horizontal_travel_dist
                            {
                                return Gesture::GestureSwipeLeft;
                            }

                            // Detect up swipe
                            if hand_pos_newer_cart.z - hand_pos_cart.z
                                > self.params.swipe_vertical_travel_dist
                            {
                                return Gesture::GestureSwipeUp;
                            }

                            // Detect down swipe
                            if hand_pos_newer_cart.z - hand_pos_cart.z
                                < -self.params.swipe_vertical_travel_dist
                            {
                                return Gesture::GestureSwipeDown;
                            }
                        }
                    }
                }
            }
        }

        Gesture::GestureNone
    }
}
