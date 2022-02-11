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
            when.method(GET).path(format!("/{}", fixture_path));
            then.status(200).header(ETAG_KEY, etag).body(&contents);
        });
        let resource_head = server.mock(|when, then| {
            when.method(HEAD).path(format!("/{}", fixture_path));
            then.status(200).header(ETAG_KEY, etag);
        });
        Fixture {
            url: server.url(format!("/{}", fixture_path)),
            get: resource_get,
            head: resource_head,
        }
    }
}

impl<'a> Drop for Fixture<'a> {
    fn drop(&mut self) {
        self.head.delete();
        self.get.delete();
    }
}

#[test]
fn test_get_cached_path_local_file() {
    // Setup cache.
    let cache_dir = tempdir().unwrap();
    let cache = Cache::builder()
        .dir(cache_dir.path().to_owned())
        .progress_bar(None)
        .build()
        .unwrap();

    let path = cache.cached_pa