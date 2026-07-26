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
                    " header is missing from the HTTP response."
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
                _request: &$crate::network::models::HttpRequest,
                response: &$crate::network::models::HttpResponse,
            ) -> bool {
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
                    "{} The '{}' HTTP response header is not present.",
                    $desc, $header
                );
                result.evidence = format!(
                    "Request URL: {}\nResponse headers present: {}",
                    request.url,
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
