#![allow(dead_code)]

use std::collections::HashMap;

use crate::sequencer::errors::SequencerError;
use crate::sequencer::models::{
    LengthStats, Observation, ObservationKind, ObservationSeverity, PositionStats, TokenAnalysis,
    TokenClass,
};

pub const MIN_SAMPLES: usize = 8;
const LOW_ENTROPY_THRESHOLD_BITS: f64 = 1.0;
const POSITION_MIN_SAMPLES: usize = 16;

pub fn shannon_entropy_bits(counts: &[usize]) -> f64 {
    let total: usize = counts.iter().sum();
    if total == 0 {
        return 0.0;
    }
    let total_f = total as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / total_f;
            -p * p.log2()
        })
        .sum()
}

pub fn shannon_entropy(samples: &[String]) -> f64 {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for token in samples {
        for c in token.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let values: Vec<usize> = counts.values().copied().collect();
    shannon_entropy_bits(&values)
}

pub fn analyze(
    samples: &[String],
    class: TokenClass,
) -> Result<TokenAnalysis, SequencerError> {
    if samples.is_empty() {
        return Err(SequencerError::Empty);
    }
    if samples.len() < MIN_SAMPLES {
        return Err(SequencerError::TooFewSamples {
            got: samples.len(),
            required: MIN_SAMPLES,
        });
    }

    let non_empty = samples.iter().filter(|s| !s.is_empty()).count();
    if non_empty == 0 {
        return Err(SequencerError::Empty);
    }

    let min_len = samples.iter().map(|s| s.chars().count()).min().unwrap_or(0);
    let max_len = samples.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    let length = LengthStats {
        min: min_len,
        max: max_len,
        fixed: min_len == max_len,
    };

    let mut unique: std::collections::HashSet<&String> = std::collections::HashSet::new();
    for s in samples {
        unique.insert(s);
    }
    let unique_count = unique.len();

    let mut alphabet: std::collections::HashSet<char> = std::collections::HashSet::new();
    for s in samples {
        alphabet.extend(s.chars());
    }
    let alphabet_size = alphabet.len().max(1);
    let ideal_entropy = (alphabet_size as f64).log2();
    let per_char_entropy = shannon_entropy(samples);
    let total_entropy_bits = per_char_entropy * min_len as f64;

    let per_position = position_stats(samples, min_len, ideal_entropy);

    let mut observations = Vec::new();
    collect_position_observations(&per_position, &mut observations);
    collect_repeat_observations(samples.len(), unique_count, &mut observations);
    if let Some(detail) = sequential_detail(samples) {
        observations.push(Observation {
            kind: ObservationKind::Sequential,
            severity: ObservationSeverity::Medium,
            detail,
        });
    }
    if let Some(detail) = fixed_affix_detail(samples, min_len) {
        observations.push(Observation {
            kind: if detail.0 { ObservationKind::FixedPrefix } else { ObservationKind::FixedSuffix },
            severity: ObservationSeverity::Low,
            detail: detail.1,
        });
    }
    if alphabet_size <= 10 && min_len >= 8 {
        observations.push(Observation {
            kind: ObservationKind::SmallAlphabet,
            severity: ObservationSeverity::Informational,
            detail: format!(
                "all {min_len}+ character samples use only {alphabet_size} distinct characters"
            ),
        });
    }
    if !length.fixed {
        observations.push(Observation {
            kind: ObservationKind::VariableLength,
            severity: ObservationSeverity::Informational,
            detail: format!(
                "token length varies between {} and {} characters",
                length.min, length.max
            ),
        });
    }

    let methodology = format!(
        "Shannon entropy computed over {} observed {}. Per-character entropy {:.3} bits (ceiling {:.3} bits for \
         an alphabet of {}); total sample entropy {:.2} bits across the shortest observed length of {}. Per-position \
         entropy reported for the first {} positions. Statistical power is limited by the sample count; \
         a low-entropy observation is reported only when a position's observed entropy is below {} bit with at \
         least {} samples. A token is never classified as weak from length or appearance alone.",
        samples.len(),
        class.label(),
        per_char_entropy,
        ideal_entropy,
        alphabet_size,
        total_entropy_bits,
        min_len,
        min_len,
        LOW_ENTROPY_THRESHOLD_BITS,
        POSITION_MIN_SAMPLES
    );

    Ok(TokenAnalysis {
        class,
        sample_count: samples.len(),
        unique_count,
        length,
        alphabet_size,
        shannon_entropy_bits_per_char: per_char_entropy,
        ideal_entropy_bits_per_char: ideal_entropy,
        total_entropy_bits,
        per_position,
        observations,
        methodology,
        confidence: statistical_confidence(samples.len()),
    })
}

fn position_stats(samples: &[String], min_len: usize, ideal: f64) -> Vec<PositionStats> {
    let mut result = Vec::with_capacity(min_len);
    for position in 0..min_len {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for s in samples {
            if let Some(c) = s.chars().nth(position) {
                *counts.entry(c).or_insert(0) += 1;
            }
        }
        let values: Vec<usize> = counts.values().copied().collect();
        let entropy = shannon_entropy_bits(&values);
        let most_common = counts
            .iter()
            .max_by_key(|(_, &c)| c)
            .map(|(&c, &count)| (c, count));
        result.push(PositionStats {
            position,
            samples: values.iter().sum(),
            distinct_chars: counts.len(),
            entropy_bits: entropy,
            max_entropy_bits: (counts.len().max(1) as f64).log2().min(ideal),
            most_common,
        });
    }
    result
}

fn collect_position_observations(per_position: &[PositionStats], out: &mut Vec<Observation>) {
    for stats in per_position {
        if stats.samples >= POSITION_MIN_SAMPLES && stats.entropy_bits < LOW_ENTROPY_THRESHOLD_BITS {
            let top = stats
                .most_common
                .map(|(c, n)| format!(" (most common character {c:?} appears {n} times)"))
                .unwrap_or_default();
            out.push(Observation {
                kind: ObservationKind::LowEntropyPosition,
                severity: ObservationSeverity::Low,
                detail: format!(
                    "position {} has {:.3} bits of entropy across {} samples{top}",
                    stats.position, stats.entropy_bits, stats.samples
                ),
            });
        }
    }
}

fn collect_repeat_observations(sample_count: usize, unique_count: usize, out: &mut Vec<Observation>) {
    let repeated = sample_count.saturating_sub(unique_count);
    if repeated > 0 {
        out.push(Observation {
            kind: ObservationKind::RepeatedValue,
            severity: if repeated * 2 >= sample_count {
                ObservationSeverity::Medium
            } else {
                ObservationSeverity::Low
            },
            detail: format!(
                "{repeated} of {sample_count} collected tokens are exact duplicates of another sample"
            ),
        });
    }
}

fn sequential_detail(samples: &[String]) -> Option<String> {
    let decimals: Option<Vec<i128>> = samples
        .iter()
        .map(|s| s.trim().parse::<i128>().ok())
        .collect();
    if let Some(nums) = decimals {
        if nums.len() >= 3 {
            let mut consecutive = true;
            let mut constant_step: Option<i128> = None;
            let mut arithmetic = true;
            for w in nums.windows(2) {
                let diff = w[1] - w[0];
                if diff != 1 {
                    consecutive = false;
                }
                match constant_step {
                    None => constant_step = Some(diff),
                    Some(step) if step != diff => arithmetic = false,
                    _ => {}
                }
            }
            if consecutive {
                return Some(
                    "every collected token parses as an integer and increases by exactly 1".into(),
                );
            }
            if arithmetic {
                if let Some(step) = constant_step {
                    return Some(format!(
                        "every collected token parses as an integer in an arithmetic sequence with a constant step of {step}"
                    ));
                }
            }
        }
    }
    None
}

fn fixed_affix_detail(samples: &[String], min_len: usize) -> Option<(bool, String)> {
    if samples.len() < 2 || min_len == 0 {
        return None;
    }
    let chars: Vec<Vec<char>> = samples.iter().map(|s| s.chars().collect()).collect();
    let mut prefix = 0;
    while prefix < min_len && chars.iter().all(|c| c.get(prefix) == chars[0].get(prefix)) {
        prefix += 1;
    }
    if prefix >= 4 && prefix >= (min_len + 2) / 3 {
        return Some((
            true,
            format!(
                "all {}/{} samples share the same {}-character prefix: {:?}",
                samples.len(),
                samples.len(),
                prefix,
                chars[0][..prefix].iter().collect::<String>()
            ),
        ));
    }
    let mut suffix = 0;
    while suffix < min_len
        && chars.iter().all(|c| c.get(c.len().wrapping_sub(1 + suffix)) == chars[0].get(chars[0].len().wrapping_sub(1 + suffix)))
    {
        suffix += 1;
    }
    if suffix >= 4 && suffix >= (min_len + 2) / 3 {
        let sample0 = &chars[0];
        return Some((
            false,
            format!(
                "all samples share the same {}-character suffix: {:?}",
                suffix,
                sample0[sample0.len() - suffix..].iter().collect::<String>()
            ),
        ));
    }
    None
}

fn statistical_confidence(sample_count: usize) -> u8 {
    match sample_count {
        0..=7 => 20,
        8..=19 => 45,
        20..=49 => 65,
        50..=99 => 80,
        100..=499 => 90,
        _ => 95,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self, bound: usize) -> usize {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 33) as usize) % bound
        }
    }

    fn high_entropy_samples(n: usize) -> Vec<String> {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = Lcg(0x9E3779B97F4A7C15);
        (0..n)
            .map(|_| {
                (0..32)
                    .map(|_| ALPHABET[rng.next(ALPHABET.len())] as char)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn high_entropy_samples_have_no_low_entropy_observations() {
        let analysis = analyze(&high_entropy_samples(200), TokenClass::Session).unwrap();
        assert!(analysis.shannon_entropy_bits_per_char > 4.5);
        assert!(!analysis
            .observations
            .iter()
            .any(|o| matches!(o.kind, ObservationKind::LowEntropyPosition)));
        assert!(!analysis
            .observations
            .iter()
            .any(|o| matches!(o.kind, ObservationKind::RepeatedValue)));
        assert_eq!(analysis.unique_count, 200);
        assert_eq!(analysis.confidence, 90);
    }

    #[test]
    fn constant_tokens_are_flagged_with_evidence() {
        let samples = vec!["AAAA".to_string(); 20];
        let analysis = analyze(&samples, TokenClass::Session).unwrap();
        assert_eq!(analysis.shannon_entropy_bits_per_char, 0.0);
        assert!(analysis
            .observations
            .iter()
            .any(|o| matches!(o.kind, ObservationKind::RepeatedValue)));
        assert!(analysis
            .observations
            .iter()
            .any(|o| matches!(o.kind, ObservationKind::LowEntropyPosition)));
    }

    #[test]
    fn sequential_numeric_tokens_are_flagged() {
        let samples: Vec<String> = (1..=40).map(|i| i.to_string()).collect();
        let analysis = analyze(&samples, TokenClass::PasswordReset).unwrap();
        assert!(analysis
            .observations
            .iter()
            .any(|o| o.kind == ObservationKind::Sequential));
    }

    #[test]
    fn short_tokens_alone_are_not_called_weak() {
        let samples: Vec<String> = ('a'..='t').map(|c| c.to_string()).collect();
        let analysis = analyze(&samples, TokenClass::Csrf).unwrap();
        assert_eq!(analysis.length.fixed, true);
        assert_eq!(analysis.length.min, 1);
        assert!(!analysis
            .observations
            .iter()
            .any(|o| o.kind == ObservationKind::Sequential));
        assert!(!analysis
            .observations
            .iter()
            .any(|o| o.kind == ObservationKind::RepeatedValue));
    }

    #[test]
    fn fixed_prefix_is_detected() {
        let samples: Vec<String> = (0..40).map(|i| format!("SESS-{i:08}")).collect();
        let analysis = analyze(&samples, TokenClass::Session).unwrap();
        assert!(analysis
            .observations
            .iter()
            .any(|o| o.kind == ObservationKind::FixedPrefix));
    }

    #[test]
    fn too_few_samples_is_rejected() {
        let samples = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            analyze(&samples, TokenClass::Generic),
            Err(SequencerError::TooFewSamples { got: 2, required: 8 })
        );
    }

    #[test]
    fn empty_is_rejected() {
        assert_eq!(analyze(&[], TokenClass::Generic), Err(SequencerError::Empty));
    }

    #[test]
    fn entropy_of_uniform_alphabet_is_log2_n() {
        let counts = vec![1usize; 256];
        let e = shannon_entropy_bits(&counts);
        assert!((e - 8.0).abs() < 1e-9);
    }
}
