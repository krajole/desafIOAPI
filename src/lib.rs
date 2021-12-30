
//! The idea behind `cached-path` is to provide a unified, simple interface for
//! accessing both local and remote files. This can be used behind other APIs that need
//! to access files agnostic to where they are located.
//!
//! This is based on the Python library [`allenai/cached_path`](https://github.com/allenai/cached_path).
//!
//! ## Installation
//!
//! `cached-path` can be used as both a library and a command-line tool. To install `cached-path`
//! as a command-line tool, run
//!
//! ```bash
//! cargo install --features build-binary cached-path
//! ```
//!
//! ## Usage
//!
//! For remote resources, `cached-path` downloads and caches the resource, using the ETAG
//! to know when to update the cache. The path returned is the local path to the latest
//! cached version:
//!
//! ```rust
//! use cached_path::cached_path;
//!
//! let path = cached_path(
//!     "https://github.com/epwalsh/rust-cached-path/blob/main/README.md"