
use std::io::{self, Write};
use std::time::Instant;

/// Progress bar types.
///
/// This can be set with
/// [`CacheBuilder::progress_bar()`](struct.CacheBuilder.html#method.progress_bar).
#[derive(Debug, Clone)]
pub enum ProgressBar {
    /// Gives pretty, verbose progress bars.
    Full,
    /// Gives progress bars with minimal output.
    ///