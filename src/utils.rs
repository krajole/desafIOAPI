use sha2::{Digest, Sha256};
use std::time::SystemTime;

pub(crate) fn hash_str(s: &s