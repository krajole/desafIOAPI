
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