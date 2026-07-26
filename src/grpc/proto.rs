use std::collections::HashMap;

use super::models::{ProtoFieldDef, ProtoMessageDef, ProtoSchema};

pub struct ProtoLoader {
    schemas: HashMap<String, ProtoSchema>,
}

impl ProtoLoader {
    pub fn new() -> Self {
        ProtoLoader {
            schemas: HashMap::new(),
        }
    }

    pub fn load_proto_text(
        &mut self,
        filename: &str,
        content: &str,
    ) -> Result<&ProtoSchema, String> {
        let schema = parse_proto_file(content, filename)?;
        self.schemas.insert(filename.to_string(), schema);
        Ok(self.schemas.get(filename).unwrap())
    }

    pub fn get_schema(&self, filename: &str) -> Option<&ProtoSchema> {
        self.schemas.get(filename)
    }

    pub fn resolve_field_name(
        &self,
        _package: Option<&str>,
        _message_name: Option<&str>,
        _field_number: u32,
    ) -> Option<String> {
        None
    }

    pub fn list_services(&self) -> Vec<String> {
        self.schemas
            .values()
            .flat_map(|s| s.services.iter().map(|svc| svc.name.clone()))
            .collect()
    }

    pub fn clear(&mut self) {
        self.schemas.clear();
    }
}

impl Default for ProtoLoader {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_proto_file(content: &str, filename: &str) -> Result<ProtoSchema, String> {
    let mut schema = ProtoSchema {
        services: Vec::new(),
        messages: Vec::new(),
        package: None,
        source_file: filename.to_string(),
    };

    let mut current_service: Option<(String, Vec<super::models::GrpcMethod>)> = None;
    let mut current_message: Option<(String, Vec<ProtoFieldDef>)> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        let commentless = trimmed.split("//").next().unwrap_or("").trim();

        if commentless.is_empty() {
            continue;
        }

        if let Some(pkg) = commentless.strip_prefix("package ") {
            schema.package = Some(pkg.trim_end_matches(';').trim().to_string());
        } else if commentless.starts_with("service ") {
            if let Some(rest) = commentless.strip_prefix("service ") {
                let name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
                current_service = Some((name, Vec::new()));
            }
        } else if commentless == "}" {
            if let Some((name, methods)) = current_service.take() {
                schema.services.push(super::models::GrpcService {
                    name,
                    package: schema.package.clone(),
                    methods,
                    source: filename.to_string(),
                });
            }
            if let Some((name, fields)) = current_message.take() {
                let full_name = match &schema.package {
                    Some(p) => format!("{}.{}", p, name),
                    None => name.clone(),
                };
                schema.messages.push(ProtoMessageDef {
                    name,
                    full_name,
                    fields,
                });
            }
        } else if commentless.starts_with("message ") {
            if let Some(rest) = commentless.strip_prefix("message ") {
                let name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
                current_message = Some((name, Vec::new()));
            }
        } else if commentless.starts_with("rpc ") {
            if let Some(ref mut svc) = current_service {
                if let Some(rest) = commentless.strip_prefix("rpc ") {
                    let parts: Vec<&str> = rest.split(&['(', ')'] as &[char]).collect();
                    if parts.len() >= 3 {
                        let method_name = parts[0].trim().to_string();
                        let input = parts[1].trim();
                        let output = if parts.len() >= 4 {
                            parts[3].trim()
                        } else {
                            input
                        };
                        let has_stream_input =
                            rest.contains("stream ") && rest.find("stream ") < rest.find('(');
                        let has_stream_output =
                            rest.contains("stream ") && rest.find("stream ") > rest.find(')');

                        svc.1.push(super::models::GrpcMethod {
                            name: method_name.clone(),
                            full_name: format!(
                                "{}.{}/{}",
                                schema.package.as_deref().unwrap_or(""),
                                svc.0,
                                method_name
                            ),
                            input_type: input.to_string(),
                            output_type: output.to_string(),
                            client_streaming: has_stream_input,
                            server_streaming: has_stream_output,
                        });
                    }
                }
            }
        } else if let Some(ref mut msg) = current_message {
            let parts: Vec<&str> = commentless.split_whitespace().collect();
            if parts.len() >= 3 && parts[parts.len() - 1].ends_with(';') {
                let label =
                    if parts[0] == "repeated" || parts[0] == "optional" || parts[0] == "required" {
                        parts[0].to_string()
                    } else {
                        "optional".to_string()
                    };
                let type_idx =
                    if parts[0] == "repeated" || parts[0] == "optional" || parts[0] == "required" {
                        1
                    } else {
                        0
                    };
                let field_type = parts
                    .get(type_idx)
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let name_part = parts.get(type_idx + 1).unwrap_or(&"");
                let name = name_part.trim_end_matches(';').trim().to_string();
                let number_str = parts
                    .get(type_idx + 3)
                    .unwrap_or(&"0")
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
                let number: u32 = number_str.parse().unwrap_or(0);

                if number > 0 && !name.is_empty() {
                    msg.1.push(ProtoFieldDef {
                        name,
                        number,
                        field_type,
                        label,
                    });
                }
            }
        }
    }

    Ok(schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_proto() {
        let proto = r#"
syntax = "proto3";
package example;

service Greeter {
    rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
    string name = 1;
}

message HelloReply {
    string message = 1;
}
"#;

        let schema = parse_proto_file(proto, "test.proto").unwrap();
        assert_eq!(schema.package.as_deref(), Some("example"));
        assert_eq!(schema.services.len(), 1);
        assert_eq!(schema.services[0].name, "Greeter");
        assert_eq!(schema.services[0].methods.len(), 1);
        assert_eq!(schema.services[0].methods[0].name, "SayHello");
        assert_eq!(schema.services[0].methods[0].input_type, "HelloRequest");
        assert_eq!(schema.services[0].methods[0].output_type, "HelloReply");
        assert_eq!(schema.messages.len(), 2);
    }

    #[test]
    fn test_proto_loader() {
        let mut loader = ProtoLoader::new();
        let proto = r#"
syntax = "proto3";
service TestSvc {
    rpc DoThing (Request) returns (Response);
}
message Request { string data = 1; }
message Response { string result = 1; }
"#;
        let result = loader.load_proto_text("test.proto", proto);
        assert!(result.is_ok());
        assert!(loader.get_schema("test.proto").is_some());
    }
}
