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

impl ArchiveFormat {
    /// Parse archive type from resource extension.
    pub(crate) fn parse_from_extension(resource: &str) -> Result<Self, Error> {
        if resource.ends_with(".tar.gz") {
            Ok(Self::TarGz)
        } else if resource.ends_with(".zip") {
            Ok(Self::Zip)
        } else {
            Err(Error::ExtractionError("unsupported archive format".into()))
        }
    }
}

pub(crate) fn extract_archive<P: AsRef<Path>>(
    path: P,
    targe