#![allow(dead_code)]

use crate::network::models::HttpRequest;
use crate::scanner::payload::encoder::apply_encoding;
use crate::scanner::payload::generator::PayloadGenerator;
use crate::scanner::payload::inserter::{insert, insert_all};
use crate::scanner::payload::models::{EncoderType, InsertionPoint, MutationType, Payload};
use crate::scanner::payload::mutator::{mutate, mutate_all};
use crate::scanner::payload::repository::PayloadRepository;

pub struct PayloadEngine {
    repository: PayloadRepository,
}

impl PayloadEngine {
    pub fn new() -> Self {
        Self {
            repository: PayloadRepository::with_defaults(),
        }
    }

    pub fn repository(&self) -> &PayloadRepository {
        &self.repository
    }
    pub fn repository_mut(&mut self) -> &mut PayloadRepository {
        &mut self.repository
    }

    pub fn generate(&self, name: &str) -> Vec<Payload> {
        self.repository
            .get(name)
            .map(|s| {
                PayloadGenerator::from_list(
                    &s.payloads.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default()
    }

    pub fn encode_payload(&self, payload: &mut Payload, encoder: EncoderType) {
        apply_encoding(payload, encoder);
    }

    pub fn encode_all(&self, payloads: &[Payload], encoders: &[EncoderType]) -> Vec<Payload> {
        let mut results = payloads.to_vec();
        for enc in encoders {
            let mut encoded: Vec<Payload> = payloads
                .iter()
                .map(|p| {
                    let mut p2 = p.clone();
                    apply_encoding(&mut p2, *enc);
                    p2
                })
                .collect();
            results.append(&mut encoded);
        }
        results
    }

    pub fn mutate_payload(&self, payload: &mut Payload, mutation: &MutationType) {
        mutate(payload, mutation);
    }

    pub fn mutate_all(&self, payloads: &[Payload], mutations: &[MutationType]) -> Vec<Payload> {
        mutate_all(&mut payloads.to_vec(), mutations)
    }

    pub fn insert_into(
        &self,
        request: &HttpRequest,
        point: &InsertionPoint,
        value: &str,
    ) -> HttpRequest {
        let mut req = request.clone();
        insert(&mut req, point, value);
        req
    }

    pub fn insert_all_points(
        &self,
        request: &HttpRequest,
        points: &[InsertionPoint],
        value: &str,
    ) -> Vec<HttpRequest> {
        insert_all(request, points, value)
    }

    pub fn generate_matrix(
        &self,
        request: &HttpRequest,
        payload_name: &str,
        insertion_points: &[InsertionPoint],
        encoders: &[EncoderType],
        mutations: &[MutationType],
    ) -> Vec<HttpRequest> {
        let payloads = self.generate(payload_name);
        if payloads.is_empty() {
            return vec![];
        }

        let encoded = if encoders.is_empty() {
            payloads
        } else {
            self.encode_all(&payloads, encoders)
        };
        let mutated = if mutations.is_empty() {
            encoded.clone()
        } else {
            self.mutate_all(&encoded, mutations)
        };
        let all_payloads = [encoded, mutated].concat();

        let mut results = Vec::new();
        for payload in &all_payloads {
            for point in insertion_points {
                results.push(self.insert_into(request, point, &payload.value));
            }
        }
        results
    }
}
