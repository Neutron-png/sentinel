#![allow(dead_code)]

use uuid::Uuid;

use crate::sitemap::builder::SiteMapBuilder;
use crate::sitemap::errors::SiteMapError;
use crate::sitemap::models::{Endpoint, Host};

pub struct SiteMapEngine {
    builder: SiteMapBuilder,
}

impl SiteMapEngine {
    pub fn new() -> Self {
        Self {
            builder: SiteMapBuilder::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn observe(
        &mut self,
        hostname: &str,
        scheme: &str,
        port: u16,
        method: &str,
        path: &str,
        status: u16,
        params: &[String],
        content_type: &str,
    ) {
        self.builder.observe_request(
            hostname,
            scheme,
            port,
            method,
            path,
            status,
            params,
            content_type,
        );
    }

    pub fn hosts(&self) -> &[Host] {
        self.builder.hosts()
    }
    pub fn host_count(&self) -> usize {
        self.builder.host_count()
    }
    pub fn endpoint_count(&self) -> usize {
        self.builder.endpoint_count()
    }

    pub fn find_host(&self, hostname: &str, scheme: &str, port: u16) -> Option<&Host> {
        self.builder.find_host(hostname, scheme, port)
    }

    pub fn find_endpoint(&self, host_id: Uuid, normalized_path: &str) -> Option<&Endpoint> {
        self.builder.find_endpoint(host_id, normalized_path)
    }

    pub fn endpoints_for_host(&self, host_id: Uuid) -> Option<&[Endpoint]> {
        self.builder
            .hosts()
            .iter()
            .find(|h| h.id == host_id)
            .map(|h| h.endpoints.as_slice())
    }

    pub fn delete_endpoint(
        &mut self,
        host_id: Uuid,
        endpoint_id: Uuid,
    ) -> Result<(), SiteMapError> {
        if let Some(host) = self.builder.hosts.iter_mut().find(|h| h.id == host_id) {
            let len = host.endpoints.len();
            host.endpoints.retain(|e| e.id != endpoint_id);
            if host.endpoints.len() < len {
                return Ok(());
            }
        }
        Err(SiteMapError::NotFound(endpoint_id.to_string()))
    }

    pub fn search(&self, query: &str) -> Vec<&Endpoint> {
        let q = query.to_lowercase();
        self.builder
            .hosts()
            .iter()
            .flat_map(|h| h.endpoints.iter())
            .filter(|e| {
                e.path.to_lowercase().contains(&q)
                    || e.normalized_path.to_lowercase().contains(&q)
                    || e.methods.iter().any(|m| m.to_lowercase().contains(&q))
                    || e.parameters.iter().any(|p| p.to_lowercase().contains(&q))
                    || e.content_types
                        .iter()
                        .any(|c| c.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn search_by_host(&self, host_query: &str) -> Vec<&Host> {
        let q = host_query.to_lowercase();
        self.builder
            .hosts()
            .iter()
            .filter(|h| h.hostname.to_lowercase().contains(&q))
            .collect()
    }

    pub fn rebuild(&mut self) {
        self.builder = SiteMapBuilder::new();
    }
}
