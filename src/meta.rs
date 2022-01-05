
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::now;
use crate::Error;

/// Holds information about a cached resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Meta {
    /// The original resource name.
    pub(crate) resource: String,
    /// Path to the cached resource.
    pub(crate) resource_path: PathBuf,
    /// Path to the serialized meta.
    pub(crate) meta_path: PathBuf,
    /// The ETAG of the resource from the time it was cached, if there was one.
    pub(crate) etag: Option<String>,
    /// Time that the freshness of this cached resource will expire.
    pub(crate) expires: Option<f64>,
    /// Time this version of the resource was cached.
    pub(crate) creation_time: f64,
}

impl Meta {
    pub(crate) fn new(
        resource: String,
        resource_path: PathBuf,
        etag: Option<String>,
        freshness_lifetime: Option<u64>,
    ) -> Meta {
        let mut expires: Option<f64> = None;
        let creation_time = now();
        if let Some(lifetime) = freshness_lifetime {
            expires = Some(creation_time + (lifetime as f64));
        }
        let meta_path = Meta::meta_path(&resource_path);
        Meta {
            resource,
            resource_path,
            meta_path,
            etag,
            expires,
            creation_time,
        }
    }

    pub(crate) fn meta_path(resource_path: &Path) -> PathBuf {
        let mut meta_path = PathBuf::from(resource_path);
        let resource_file_name = meta_path.file_name().unwrap().to_str().unwrap();
        let meta_file_name = format!("{}.meta", resource_file_name);
        meta_path.set_file_name(&meta_file_name[..]);
        meta_path
    }

    pub(crate) fn get_extraction_path(&self) -> PathBuf {
        let dirname = format!(
            "{}-extracted",