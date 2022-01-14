use crate::{meta::Meta, Cache, Options};
use httpmock::Method::{GET, HEAD};
use httpmock::{MockRef, MockServer};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile::tempdir;

static ETAG_KEY: &str = "ETag";

struct Fixture<'a> {
    url: String,
    get: MockRef<'a>,
    head: MockRef<'a>,
}

impl<'