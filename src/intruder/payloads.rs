#![allow(dead_code)]

use base64::Engine;
use std::io::Read;

use crate::intruder::errors::IntruderError;
use crate::intruder::models::{PayloadEncoding, PayloadSet};

pub const MIN_LEN_LIMIT: usize = 1;
pub const MAX_LEN_LIMIT: usize = 256;
pub const MAX_LIST_SIZE: usize = 200_000;

fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for b in value.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn apply_encoding(value: &str, encoding: PayloadEncoding) -> Result<String, IntruderError> {
    match encoding {
        PayloadEncoding::None => Ok(value.to_string()),
        PayloadEncoding::Url => Ok(percent_encode(value)),
        PayloadEncoding::Base64 => Ok(base64::engine::general_purpose::STANDARD.encode(value)),
        PayloadEncoding::Hex => Ok(value
            .as_bytes()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()),
    }
}

pub fn expand(
    set: &PayloadSet,
    limit: usize,
) -> Result<Vec<String>, IntruderError> {
    match set {
        PayloadSet::List(items) => {
            if items.len() > MAX_LIST_SIZE {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "list payload set has {} items (limit {MAX_LIST_SIZE})",
                    items.len()
                )));
            }
            Ok(items.iter().map(|i| i.trim_end_matches('\r')).map(|i| i.to_string()).collect())
        }
        PayloadSet::Numeric { start, end, step } => {
            if *start > *end {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "numeric range start {start} is greater than end {end}"
                )));
            }
            if *step <= 0 {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "numeric step must be positive (got {step})"
                )));
            }
            let mut payloads = Vec::new();
            let mut current = *start;
            while current <= *end {
                payloads.push(current.to_string());
                current += *step;
                if payloads.len() as u64 > u64::try_from(limit).unwrap_or(u64::MAX) {
                    return Err(IntruderError::InvalidPayloadSet(format!(
                        "numeric range exceeds expansion limit of {limit}"
                    )));
                }
            }
            Ok(payloads)
        }
        PayloadSet::Charset { alphabet, min_len, max_len } => {
            if alphabet.is_empty() {
                return Err(IntruderError::InvalidPayloadSet(
                    "charset payload set has an empty alphabet".into(),
                ));
            }
            if *min_len < MIN_LEN_LIMIT {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "charset min_len must be at least {MIN_LEN_LIMIT}"
                )));
            }
            if *max_len > MAX_LEN_LIMIT || *max_len < *min_len {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "charset max_len must be between {min_len} and {MAX_LEN_LIMIT}"
                )));
            }
            let alphabet: Vec<char> = alphabet.chars().collect();
            let mut total: usize = 0;
            for len in *min_len..=*max_len {
                let capacity = alphabet
                    .len()
                    .checked_pow(len as u32)
                    .unwrap_or(usize::MAX);
                total = total.saturating_add(capacity);
                if total > limit {
                    return Err(IntruderError::InvalidPayloadSet(format!(
                        "charset expansion exceeds limit of {limit}"
                    )));
                }
            }
            let mut payloads = Vec::new();
            for len in *min_len..=*max_len {
                let alphabet_len = alphabet.len();
                let mut indices = vec![0usize; len];
                loop {
                    payloads.push(indices.iter().map(|&i| alphabet[i]).collect());
                    let mut carry = false;
                    let mut p = len;
                    loop {
                        if p == 0 {
                            carry = true;
                            break;
                        }
                        p -= 1;
                        if indices[p] + 1 < alphabet_len {
                            indices[p] += 1;
                            break;
                        }
                        indices[p] = 0;
                    }
                    if carry {
                        break;
                    }
                }
            }
            Ok(payloads)
        }
        PayloadSet::File { path } => {
            let mut file = std::fs::File::open(path)
                .map_err(|e| IntruderError::Io(format!("{}: {e}", path)))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| IntruderError::Io(format!("{}: {e}", path)))?;
            let items: Vec<String> = contents
                .lines()
                .map(|l| l.trim_end_matches('\r').to_string())
                .filter(|l| !l.is_empty() && !l.trim_start().starts_with('#'))
                .take(MAX_LIST_SIZE)
                .collect();
            if items.len() >= MAX_LIST_SIZE {
                return Err(IntruderError::InvalidPayloadSet(format!(
                    "file payload set exceeds limit of {MAX_LIST_SIZE} items"
                )));
            }
            Ok(items)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_expands_inclusive() {
        let set = PayloadSet::Numeric { start: 1, end: 5, step: 2 };
        let payloads = expand(&set, 100).unwrap();
        assert_eq!(payloads, vec!["1", "3", "5"]);
    }

    #[test]
    fn numeric_rejects_bad_ranges() {
        assert!(expand(&PayloadSet::Numeric { start: 5, end: 1, step: 1 }, 100).is_err());
        assert!(expand(&PayloadSet::Numeric { start: 1, end: 5, step: 0 }, 100).is_err());
    }

    #[test]
    fn list_expands_and_trims() {
        let set = PayloadSet::List(vec!["a".into(), "b ".into()]);
        let payloads = expand(&set, 100).unwrap();
        assert_eq!(payloads.len(), 2);
    }

    #[test]
    fn encoding_roundtrips() {
        assert_eq!(apply_encoding("a b", PayloadEncoding::Url).unwrap(), "a%20b");
        assert_eq!(apply_encoding("ab\u{14}\u{23}", PayloadEncoding::Hex).unwrap(), "61621423");
        assert_eq!(
            apply_encoding("ab:", PayloadEncoding::Base64).unwrap(),
            base64::engine::general_purpose::STANDARD.encode("ab:")
        );
        assert_eq!(apply_encoding("x", PayloadEncoding::None).unwrap(), "x");
    }

    #[test]
    fn charset_small_expands() {
        let set = PayloadSet::Charset { alphabet: "ab".into(), min_len: 2, max_len: 2 };
        let payloads = expand(&set, 100).unwrap();
        assert_eq!(payloads, vec!["aa", "ab", "ba", "bb"]);
    }
}
