#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;

use rcgen::{
    BasicConstraints, CertificateParams, CertifiedKey, DistinguishedName, DnType, IsCa, KeyPair,
    KeyUsagePurpose,
};

use crate::proxy::errors::ProxyError;

pub struct CertificateAuthority {
    ca_cert: CertifiedKey,
    ca_pem: String,
    cert_cache: Mutex<HashMap<String, (String, String)>>,
}

impl CertificateAuthority {
    pub fn generate() -> Result<Self, ProxyError> {
        let mut params = CertificateParams::default();
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "Sentinel Proxy CA");
        dn.push(DnType::OrganizationName, "Sentinel");
        params.distinguished_name = dn;
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        params.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::CrlSign,
            KeyUsagePurpose::DigitalSignature,
        ];

        let key_pair = KeyPair::generate().map_err(|e| ProxyError::Cert(e.to_string()))?;
        let cert = params
            .self_signed(&key_pair)
            .map_err(|e| ProxyError::Cert(e.to_string()))?;

        let ca_pem = cert.pem();
        let ca_cert = CertifiedKey { cert, key_pair };

        Ok(Self {
            ca_cert,
            ca_pem,
            cert_cache: Mutex::new(HashMap::new()),
        })
    }

    pub fn ca_pem(&self) -> &str {
        &self.ca_pem
    }

    pub fn generate_leaf(&self, hostname: &str) -> Result<(String, String), ProxyError> {
        {
            let cache = self.cert_cache.lock().unwrap();
            if let Some(cached) = cache.get(hostname) {
                return Ok(cached.clone());
            }
        }

        let mut params = CertificateParams::new(vec![hostname.to_string()])
            .map_err(|e| ProxyError::Cert(e.to_string()))?;
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, hostname);
        params.distinguished_name = dn;

        let leaf_key = KeyPair::generate().map_err(|e| ProxyError::Cert(e.to_string()))?;
        let leaf_cert = params
            .signed_by(&leaf_key, &self.ca_cert.cert, &self.ca_cert.key_pair)
            .map_err(|e| ProxyError::Cert(e.to_string()))?;

        let cert_pem = leaf_cert.pem();
        let key_pem = leaf_key.serialize_pem();

        let mut cache = self.cert_cache.lock().unwrap();
        cache.insert(hostname.to_string(), (cert_pem.clone(), key_pem.clone()));

        Ok((cert_pem, key_pem))
    }
}
