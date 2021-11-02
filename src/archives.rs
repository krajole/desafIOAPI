use crate::error::Error;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::path::Path;
use tempfile::tempdir_in;

/// Supported archive types.
pub(crate) enum ArchiveFormat {
    TarGz,
    Zip,
}

im