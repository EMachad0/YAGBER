mod edge_detector;
mod timer;

#[macro_use]
extern crate tracing;

pub use edge_detector::{EdgeDetector, EdgeMode};
pub use timer::Timer;
