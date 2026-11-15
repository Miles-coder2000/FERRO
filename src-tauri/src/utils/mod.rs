// Logging support module
// Links to src/utils/logging.rs and re-exports public API

use std::path::PathBuf;
mod logging; // Links to src/utils/logging.rs

/// Initialize the tracing subscriber with environment-based filtering.
pub use logging::log_dir;

/// Convenience: returns the resolved log directory path.
pub fn get_log_dir() -> PathBuf {
    logging::log_dir()
}