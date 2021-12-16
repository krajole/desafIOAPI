
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
    /// Construct a new `CacheBuilder`.
    pub fn new() -> CacheBuilder {
        CacheBuilder {
            config: Config {
                dir: None,
                client_builder: ClientBuilder::new().timeout(None),
                max_retries: 3,
                max_backoff: 5000,
                freshness_lifetime: None,
                offline: false,
                progress_bar: Some(ProgressBar::default()),
            },
        }
    }

    /// Construct a new `CacheBuilder` with a `ClientBuilder`.
    pub fn with_client_builder(client_builder: ClientBuilder) -> CacheBuilder {
        CacheBuilder::new().client_builder(client_builder)
    }

    /// Set the cache location. This can be set through the environment
    /// variable `RUST_CACHED_PATH_ROOT`. Otherwise it will default to a subdirectory
    /// named 'cache' of the default system temp directory.
    pub fn dir(mut self, dir: PathBuf) -> CacheBuilder {
        self.config.dir = Some(dir);
        self
    }

    /// Set the `ClientBuilder`.
    pub fn client_builder(mut self, client_builder: ClientBuilder) -> CacheBuilder {
        self.config.client_builder = client_builder;
        self
    }

    /// Enable a request timeout.
    pub fn timeout(mut self, timeout: Duration) -> CacheBuilder {
        self.config.client_builder = self.config.client_builder.timeout(timeout);
        self
    }

    /// Enable a timeout for the connect phase of each HTTP request.
    pub fn connect_timeout(mut self, timeout: Duration) -> CacheBuilder {
        self.config.client_builder = self.config.client_builder.connect_timeout(timeout);
        self
    }

    /// Set maximum number of retries for HTTP requests.
    pub fn max_retries(mut self, max_retries: u32) -> CacheBuilder {
        self.config.max_retries = max_retries;
        self
    }

    /// Set the maximum backoff delay in milliseconds for retrying HTTP requests.
    pub fn max_backoff(mut self, max_backoff: u32) -> CacheBuilder {
        self.config.max_backoff = max_backoff;
        self
    }

    /// Set the default freshness lifetime, in seconds. The default is None, meaning
    /// the ETAG for an external resource will always be checked for a fresher value.
    pub fn freshness_lifetime(mut self, freshness_lifetime: u64) -> CacheBuilder {
        self.config.freshness_lifetime = Some(freshness_lifetime);
        self
    }

    /// Only use offline functionality.
    ///
    /// If set to `true`, when the cached path of an HTTP resource is requested,
    /// the latest cached version is returned without checking for freshness.
    /// But if no cached versions exist, a
    /// [`NoCachedVersions`](enum.Error.html#variant.NoCachedVersions) error is returned.
    pub fn offline(mut self, offline: bool) -> CacheBuilder {
        self.config.offline = offline;
        self
    }

    /// Set the type of progress bar to use.
    ///
    /// The default is `Some(ProgressBar::Full)`.
    pub fn progress_bar(mut self, progress_bar: Option<ProgressBar>) -> CacheBuilder {
        self.config.progress_bar = progress_bar;
        self
    }

    /// Build the `Cache` object.
    pub fn build(self) -> Result<Cache, Error> {
        let dir = self.config.dir.unwrap_or_else(|| {
            if let Some(dir_str) = env::var_os("RUST_CACHED_PATH_ROOT") {
                PathBuf::from(dir_str)
            } else {
                env::temp_dir().join("cache/")
            }
        });
        let http_client = self.config.client_builder.build()?;
        fs::create_dir_all(&dir)?;
        Ok(Cache {
            dir,
            http_client,
            max_retries: self.config.max_retries,
            max_backoff: self.config.max_backoff,
            freshness_lifetime: self.config.freshness_lifetime,
            offline: self.config.offline,
            progress_bar: self.config.progress_bar,
        })
    }
}

impl Default for CacheBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Options to use with [`Cache::cached_path_with_options`].
#[derive(Default)]
pub struct Options {
    /// An optional subdirectory (relative to the cache root) to cache the resource in.
    pub subdir: Option<String>,
    /// Automatically extract the resource, assuming the resource is an archive.
    pub extract: bool,
}

impl Options {
    pub fn new(subdir: Option<&str>, extract: bool) -> Self {
        Self {
            subdir: subdir.map(String::from),
            extract,
        }
    }

    /// The the cache subdirectory to use.
    pub fn subdir(mut self, subdir: &str) -> Self {
        self.subdir = Some(subdir.into());
        self
    }

    /// Treat the resource as an archive and try to extract it.
    pub fn extract(mut self) -> Self {
        self.extract = true;
        self
    }
}

/// Fetches and manages resources in a local cache directory.
#[derive(Debug, Clone)]
pub struct Cache {
    /// The root directory of the cache.
    pub dir: PathBuf,
    /// The maximum number of times to retry downloading a remote resource.
    max_retries: u32,
    /// The maximum amount of time (in milliseconds) to wait before retrying a download.
    max_backoff: u32,
    /// An optional freshness lifetime (in seconds).
    ///
    /// If set, resources that were cached within the past `freshness_lifetime` seconds
    /// will always be regarded as fresh, and so the ETag of the corresponding remote
    /// resource won't be checked.
    freshness_lifetime: Option<u64>,
    /// Offline mode.
    ///
    /// If set to `true`, no HTTP calls will be made.
    offline: bool,
    /// The verbosity level of the progress bar.
    progress_bar: Option<ProgressBar>,
    /// The HTTP client used to fetch remote resources.
    http_client: Client,
}

impl Cache {
    /// Create a new `Cache` instance.
    pub fn new() -> Result<Self, Error> {
        Cache::builder().build()
    }

    /// Create a `CacheBuilder`.
    pub fn builder() -> CacheBuilder {
        CacheBuilder::new()
    }

    /// Get the cached path to a resource.
    ///
    /// If the resource is local file, it's path is returned. If the resource is a static HTTP
    /// resource, it will cached locally and the path to the cache file will be returned.
    pub fn cached_path(&self, resource: &str) -> Result<PathBuf, Error> {
        self.cached_path_with_options(resource, &Options::default())
    }
