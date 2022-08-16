use sha2::{Digest, Sha256};
use std::time::SystemTime;

pub(crate) fn hash_str(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes(