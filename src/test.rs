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

impl<'a> Fixture<'a> {
    fn load(server: &'a MockServer, fixture_path: &'a str, etag: &'a str) -> Self {
        let mut local_path = PathBuf::new();
        local_path.push(".");
        for part in fixture_path.split('/') {
            local_path.push(part);
        }
        let contents = fs::read_to_string(&local_path).unwrap();
        let resource_get = server.mock(|when, then| {
            when.method(