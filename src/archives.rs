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
    target: P,
    format: &ArchiveFormat,
) -> Result<(), Error> {
    // We'll first extract to a temp directory in the same parent as the target directory.
    let target_parent_dir = target.as_ref().parent().unwrap();
    let temp_target = tempdir_in(target_parent_dir)?;

    match format {
        ArchiveFormat::TarGz => {
            let tar_gz = File::ope