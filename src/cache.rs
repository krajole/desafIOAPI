
use fs2::FileExt;
use glob::glob;
use log::{debug, error, info, warn};
use rand::distributions::{Distribution, Uniform};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::ETAG;
use std::default::Default;
use std::env;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{self, Duration};
use tempfile::NamedTempFile;

use crate::archives::{extract_archive, ArchiveFormat};
use crate::utils::hash_str;
use crate::{meta::Meta, Error, ProgressBar};

/// Builder to facilitate creating [`Cache`] objects.
#[derive(Debug)]
pub struct CacheBuilder {
    config: Config,
}

#[derive(Debug)]
struct Config {
    dir: Option<PathBuf>,
    client_builder: ClientBuilder,
    max_retries: u32,
    max_backoff: u32,
    freshness_lifetime: Option<u64>,
    offline: bool,
    progress_bar: Option<ProgressBar>,
}

impl CacheBuilder {