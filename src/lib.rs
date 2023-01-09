#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! wave
//!
//! A no-std no-alloc gesture recognition library for low resolution TOF-Sensors

// Only enable std for tests
#![cfg_attr(not(test), no_std)]

pub mod cbind;
pub mod math;
pub mod measurements;
pub mod recognizer;

// Re-exports
pub use measurements::HandState;
pub use measurements::SensorMeasurement;
pub use recognizer::Gesture;
pub use recognizer::GestureRecognizer;
pub use recognizer::RecognizerParams;
pub use recognizer::RecognizerResult;
pub use recognizer::RecognizerStatus;

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
