#![allow(dead_code)]

use sha2::{Digest, Sha256};

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn verify(item: &super::models::EvidenceItem) -> bool {
    if item.sha256_hash.is_empty() {
        return false;
    }
    sha256(&item.content) == item.sha256_hash
}
