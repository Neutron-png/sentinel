#[macro_export]
macro_rules! define_header_rule {
    ($name:ident, $id:literal, $display:literal, $header:literal, $desc:literal, $ref:literal, $sev:expr, $conf:expr) => {
        pub struct $name;

        impl $crate::scanner::passive::rule::ScanRule for $name {
            fn id(&self) -> &'static str {
                $id
            }
            fn name(&self) -> &'static str {
                $display
            }
            fn description(&self) -> &'static str {
                concat!(
                    $desc,
                    " The ",
                    $header,
                    " header is missing from an HTML document response."
                )
            }
            fn category(&self) -> &'static str {
                "Security Headers"
            }
            fn severity(&self) -> $crate::scanner::passive::models::ScanSeverity {
                $sev
            }
            fn confidence(&self) -> $crate::scanner::passive::models::ScanConfidence {
                $conf
            }
            fn references(&self) -> &'static str {
                $ref
            }

            fn applies_to(
                &self,
                request: &$crate::network::models::HttpRequest,
                response: &$crate::network::models::HttpResponse,
            ) -> bool {
                if !$crate::scanner::passive::context::is_document_success(response) {
                    return false;
                }
                if $header == "Strict-Transport-Security"
                    && !$crate::scanner::passive::context::is_https(request)
                {
                    return false;
                }
                !response
                    .headers
                    .iter()
                    .any(|h| h.name.eq_ignore_ascii_case($header))
            }

            fn analyze(
                &self,
                transaction_id: uuid::Uuid,
                request: &$crate::network::models::HttpRequest,
                response: &$crate::network::models::HttpResponse,
            ) -> Vec<$crate::scanner::passive::models::ScanResult> {
                let mut result = $crate::scanner::passive::models::ScanResult::new(
                    self.id(),
                    transaction_id,
                    $display,
                    self.severity(),
                    self.confidence(),
                );
                result.description = format!(
                    "{} The '{}' HTTP response header is not present on an HTML document response \
                     (status {}, content-type {}). This is a configuration observation; whether it \
                     represents an exploitable weakness depends on the application context and must \
                     be confirmed manually.",
                    $desc,
                    $header,
                    response.status_code,
                    response
                        .content_type
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string())
                );
                result.evidence = format!(
                    "Request: {} {}\nStatus: {}\nContent-Type: {}\nResponse headers present: {}",
                    request.method,
                    request.url,
                    response.status_code,
                    response
                        .content_type
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    response
                        .headers
                        .iter()
                        .map(|h| h.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                result.references = self.references().to_string();
                vec![result]
            }
        }
    };
}
