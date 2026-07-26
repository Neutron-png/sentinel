#![allow(dead_code)]

use chrono::Utc;
use uuid::Uuid;

use crate::sitemap::models::{Endpoint, Host, TransactionReference};
use crate::sitemap::normalizer::PathNormalizer;

pub struct SiteMapBuilder {
    pub(crate) hosts: Vec<Host>,
    normalizer: PathNormalizer,
}

impl SiteMapBuilder {
    pub fn new() -> Self {
        Self {
            hosts: Vec::new(),
            normalizer: PathNormalizer::with_defaults(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn observe_request(
        &mut self,
        hostname: &str,
        scheme: &str,
        port: u16,
        method: &str,
        path: &str,
        status_code: u16,
        params: &[String],
        content_type: &str,
    ) {
        let now = Utc::now();
        let normalized = self.normalizer.normalize(path);

        let host = self.find_or_create_host(hostname, scheme, port);
        host.last_seen = now;

        if let Some(endpoint) = host
            .endpoints
            .iter_mut()
            .find(|e| e.normalized_path == normalized)
        {
            endpoint.methods.insert(method.to_string());
            endpoint.status_codes.push(status_code);
            endpoint.response_count += 1;
            endpoint.last_seen = now;
            if !content_type.is_empty() {
                endpoint.content_types.insert(content_type.to_string());
            }
            for p in params {
                endpoint.parameters.insert(p.clone());
            }
            endpoint.transactions.push(TransactionReference {
                transaction_id: Uuid::new_v4(),
                timestamp: now,
                method: method.to_string(),
                status_code,
            });
        } else {
            let mut ep = Endpoint::new(host.id, path, &normalized);
            ep.methods.insert(method.to_string());
            ep.status_codes.push(status_code);
            ep.request_count = 1;
            ep.response_count = 1;
            if !content_type.is_empty() {
                ep.content_types.insert(content_type.to_string());
            }
            for p in params {
                ep.parameters.insert(p.clone());
            }
            ep.transactions.push(TransactionReference {
                transaction_id: Uuid::new_v4(),
                timestamp: now,
                method: method.to_string(),
                status_code,
            });
            host.endpoints.push(ep);
        }
    }

    fn find_or_create_host(&mut self, hostname: &str, scheme: &str, port: u16) -> &mut Host {
        if let Some(pos) = self
            .hosts
            .iter()
            .position(|h| h.hostname == hostname && h.scheme == scheme && h.port == port)
        {
            return &mut self.hosts[pos];
        }
        self.hosts.push(Host::new(hostname, scheme, port));
        self.hosts.last_mut().unwrap()
    }

    pub fn hosts(&self) -> &[Host] {
        &self.hosts
    }

    pub fn find_host(&self, hostname: &str, scheme: &str, port: u16) -> Option<&Host> {
        self.hosts
            .iter()
            .find(|h| h.hostname == hostname && h.scheme == scheme && h.port == port)
    }

    pub fn find_endpoint(&self, host_id: Uuid, normalized_path: &str) -> Option<&Endpoint> {
        self.hosts.iter().find(|h| h.id == host_id).and_then(|h| {
            h.endpoints
                .iter()
                .find(|e| e.normalized_path == normalized_path)
        })
    }

    pub fn host_count(&self) -> usize {
        self.hosts.len()
    }
    pub fn endpoint_count(&self) -> usize {
        self.hosts.iter().map(|h| h.endpoints.len()).sum()
    }
}
