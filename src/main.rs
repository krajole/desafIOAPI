
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