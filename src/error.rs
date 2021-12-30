
use thiserror::Error;

/// Errors that can occur during caching.
#[derive(Error, Debug)]
pub enum Error {
    /// Arises when the resource looks like a local file but it doesn't exist.
    #[error("Treated resource as local file, but file does not exist ({0})")]
    ResourceNotFound(String),

    /// Arises when the resource looks like a URL, but is invalid.
    #[error("Unable to parse resource URL ({0})")]
    InvalidUrl(String),

    /// Arises when the cache is being used in offline mode, but it couldn't locate
    /// any cached versions of a remote resource.
    #[error("Offline mode is enabled but no cached versions of resouce exist ({0})")]
    NoCachedVersions(String),

    /// Arises when the cache is corrupted for some reason.
    ///
    /// If this error occurs, it is almost certainly the result of an external process
    /// "messing" with the cache directory, since `cached-path` takes great care
    /// to avoid accidental corruption on its own.
    #[error("Cache is corrupted ({0})")]
    CacheCorrupted(String),