#![allow(dead_code)]

use chrono::Utc;
use uuid::Uuid;

use crate::browser::network::classifier::ResourceClassifier;
use crate::browser::network::correlator::{CorrelationInfo, RequestCorrelator};
use crate::browser::network::events::{ObserverEvent, ObserverEventBus};
use crate::browser::network::models::{BrowserNetworkRequest, BrowserNetworkResponse};

pub struct BrowserNetworkObserver {
    event_bus: ObserverEventBus,
    correlator: RequestCorrelator,
}

impl BrowserNetworkObserver {
    pub fn new() -> Self {
        Self {
            event_bus: ObserverEventBus::new(1024),
            correlator: RequestCorrelator::new(),
        }
    }

    pub fn event_bus(&self) -> ObserverEventBus {
        self.event_bus.clone()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn observe_request(
        &mut self,
        browser_id: Uuid,
        tab_id: Uuid,
        method: &str,
        url: &str,
        headers: &str,
        body: &str,
        initiator: &str,
    ) -> BrowserNetworkRequest {
        let parsed =
            url::Url::parse(url).unwrap_or_else(|_| url::Url::parse("http://unknown/").unwrap());
        let resource_type = ResourceClassifier::classify(url, None, initiator);
        let request = BrowserNetworkRequest {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            method: method.to_string(),
            url: url.to_string(),
            scheme: parsed.scheme().to_string(),
            host: parsed.host_str().unwrap_or("").to_string(),
            port: parsed.port().unwrap_or(443),
            path: parsed.path().to_string(),
            query: parsed.query().unwrap_or("").to_string(),
            headers: headers.to_string(),
            cookies: String::new(),
            body: body.to_string(),
            resource_type,
            initiator: initiator.to_string(),
            frame_id: None,
            tab_id,
            browser_id,
        };
        self.correlator.add_correlation(CorrelationInfo {
            request_id: request.id,
            browser_id,
            tab_id,
            history_entry_id: None,
            sitemap_endpoint_id: None,
            proxy_connection_id: None,
        });
        self.event_bus.emit(ObserverEvent::RequestStarted {
            request: request.clone(),
        });
        self.event_bus.emit(ObserverEvent::ResourceDiscovered {
            request_id: request.id,
            resource_type,
            url: url.to_string(),
        });
        request
    }

    #[allow(clippy::too_many_arguments)]
    pub fn observe_response(
        &mut self,
        request_id: Uuid,
        status_code: u16,
        headers: &str,
        mime_type: Option<&str>,
        content_length: Option<u64>,
        timing_ms: u64,
        redirect_chain: &[String],
    ) {
        let response = BrowserNetworkResponse {
            request_id,
            status_code,
            headers: headers.to_string(),
            cookies: String::new(),
            mime_type: mime_type.map(|s| s.to_string()),
            content_length,
            encoding: None,
            timing_ms,
            redirect_chain: redirect_chain.to_vec(),
        };
        self.event_bus.emit(ObserverEvent::ResponseReceived {
            request_id,
            response: response.clone(),
        });
        self.event_bus.emit(ObserverEvent::RequestFinished {
            request_id,
            timing_ms,
        });
    }

    pub fn observe_redirect(&mut self, request_id: Uuid, from: &str, to: &str) {
        self.event_bus.emit(ObserverEvent::RedirectOccurred {
            request_id,
            from_url: from.to_string(),
            to_url: to.to_string(),
        });
    }

    pub fn observe_failure(&mut self, request_id: Uuid, error: &str) {
        self.event_bus.emit(ObserverEvent::RequestFailed {
            request_id,
            error: error.to_string(),
        });
    }

    pub fn link_history(&mut self, request_id: Uuid, history_id: Uuid) {
        self.correlator.link_history(request_id, history_id);
    }

    pub fn link_sitemap(&mut self, request_id: Uuid, endpoint_id: Uuid) {
        self.correlator.link_sitemap(request_id, endpoint_id);
    }

    pub fn correlation(&self, request_id: Uuid) -> Option<&CorrelationInfo> {
        self.correlator.find_by_request(request_id)
    }
}
