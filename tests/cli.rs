
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command; // Run programs
use tempfile::tempdir;