
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command; // Run programs
use tempfile::tempdir;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cached-path")?;
    let cache_dir = tempdir().unwrap().path().to_owned();

    cmd.arg("--dir")
        .arg(cache_dir.to_str().unwrap())
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("file does not exist"));

    Ok(())
}

#[test]
fn test_remote_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cached-path")?;
    let cache_dir = tempdir().unwrap().path().to_owned();