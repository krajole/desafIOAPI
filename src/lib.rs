
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
//! ).unwrap();
//! assert!(path.is_file());
//! ```
//!
//! ```bash
//! # From the command line:
//! $ cached-path https://github.com/epwalsh/rust-cached-path/blob/main/README.md
//! /tmp/cache/055968a99316f3a42e7bcff61d3f590227dd7b03d17e09c41282def7c622ba0f.efa33e7f611ef2d163fea874ce614bb6fa5ab2a9d39d5047425e39ebe59fe782
//! ```
//!
//! For local files, the path returned is just the original path supplied:
//!
//! ```rust
//! use cached_path::cached_path;
//!
//! let path = cached_path("README.md").unwrap();
//! assert_eq!(path.to_str().unwrap(), "README.md");
//! ```
//!
//! ```bash
//! # From the command line:
//! $ cached-path README.md
//! README.md
//! ```
//!
//! For resources that are archives, like `*.tar.gz` files, `cached-path` can also
//! automatically extract the files:
//!
//! ```rust