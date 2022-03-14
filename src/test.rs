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

    let path = cache.cached_path("README.md").unwrap();
    assert_eq!(path, Path::new("README.md"));
}

#[test]
fn test_get_cached_path_non_existant_local_file_fails() {
    // Setup cache.
    let cache_dir = tempdir().unwrap();
    let cache = Cache::builder()
        .dir(cache_dir.path().to_owned())
        .progress_bar(None)
        .build()
        .unwrap();

    let result = cache.cached_path("BLAH");
    assert!(result.is_err());
}

#[test]
fn test_cached_path_remote_file() {
    // For debugging:
    // let _ = env_logger::try_init();
    let server = MockServer::start();

    // Setup cache.
    let cache_dir = tempdir().unwrap();
    let cache = Cache::builder()
        .dir(cache_dir.path().to_owned())
        .progress_bar(None)
        .freshness_lifetime(300)
        .build()
        .unwrap();

    // Mock the resource.
    let fixture = Fixture::load(&server, "test_fixtures/hello.txt", "fake-etag");
    let resource = fixture.url.as_str();

    // Get the cached path.
    let path = cache.cached_path(resource).unwrap();

    assert_eq!(fixture.head.hits(), 1);
    assert_eq!(fixture.get.hits(), 1);

    // Ensure the file and meta exist.
    assert!(path.is_file());
    assert!(Meta::meta_path(&path).is_file());
    let mut meta = Meta::from_cache(&path).unwrap();
    assert_eq!(meta.etag.as_deref(), Some("fake-etag"));

    // Ensure the contents of the file are correct.
    let contents = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
    assert_eq!(&contents, "Hello, World!\n");

    // When we attempt to get the resource again, the cache should still be fresh.
    assert!(meta.is_fresh(None));
    let same_path = cache.cached_path(resource).unwrap();
    ass