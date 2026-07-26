#![allow(dead_code)]

use rand::Rng;

use crate::scanner::payload::models::{MutationType, Payload};

pub fn mutate(payload: &mut Payload, mutation: &MutationType) {
    match mutation {
        MutationType::Prefix(p) => {
            payload.value = format!("{}{}", p, payload.value);
        }
        MutationType::Suffix(s) => {
            payload.value = format!("{}{}", payload.value, s);
        }
        MutationType::Replace(from, to) => {
            payload.value = payload.value.replace(from, to);
        }
        MutationType::Wrap(pre, post) => {
            payload.value = format!("{}{}{}", pre, payload.value, post);
        }
        MutationType::Duplicate => {
            payload.value = format!("{}{}", payload.value, payload.value);
        }
        MutationType::RandomCase => {
            payload.value = random_case(&payload.value);
        }
        MutationType::RandomPadding(n) => {
            payload.value = pad_random(&payload.value, *n);
        }
    }
    payload.mutation = Some(format!("{:?}", mutation));
}

pub fn mutate_all(payloads: &mut [Payload], mutations: &[MutationType]) -> Vec<Payload> {
    if mutations.is_empty() {
        return payloads.to_vec();
    }
    let mut results = payloads.to_vec();
    for m in mutations {
        let new_batch: Vec<Payload> = payloads
            .iter()
            .map(|p| {
                let mut p2 = p.clone();
                mutate(&mut p2, m);
                p2
            })
            .collect();
        results.extend(new_batch);
    }
    results
}

fn random_case(s: &str) -> String {
    let mut rng = rand::thread_rng();
    s.chars()
        .map(|c| {
            if rng.gen_bool(0.5) {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn pad_random(s: &str, n: usize) -> String {
    let mut rng = rand::thread_rng();
    let padding: String = (0..n).map(|_| rng.gen_range('a'..='z')).collect();
    format!("{}{}{}", padding, s, padding)
}
