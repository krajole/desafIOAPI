
use cached_path::{Cache, Error, Options, ProgressBar};
use color_eyre::eyre::Result;
use log::debug;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cached-path",
    about = "Get the cached path to a resource.",
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
struct Opt {
    #[structopt()]
    /// The resource path.
    resource: String,

    #[structopt(long = "dir", env = "RUST_CACHED_PATH_ROOT")]
    /// The cache directory. Defaults to a subdirectory named 'cache' of the default
    /// system temporary directory.
    dir: Option<PathBuf>,

    #[structopt(long = "subdir")]
    /// The subdirectory, relative to the cache root directory to use.
    subdir: Option<String>,

    #[structopt(long = "extract")]
    /// Extract the resource as an archive.
    extract: bool,

    #[structopt(long = "timeout")]
    /// Set a request timeout.
    timeout: Option<u64>,

    #[structopt(long = "connect-timeout")]
    /// Set a timeout for the connect phase of the HTTP client.
    connect_timeout: Option<u64>,

    #[structopt(long = "max-retries", default_value = "3")]
    /// Set the maximum number of times to retry an HTTP request. Retriable failures are tried
    /// again with exponential backoff.
    max_retries: u32,

    #[structopt(long = "max-backoff", default_value = "5000")]