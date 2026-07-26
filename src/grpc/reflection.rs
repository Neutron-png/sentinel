use serde::{Deserialize, Serialize};

use super::models::{GrpcMethod, GrpcService, ProtobufValue};

const REFLECTION_SERVICE: &str = "grpc.reflection.v1alpha.ServerReflection";
const LIST_SERVICES_METHOD: &str = "ServerReflectionInfo";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReflectionRequest {
    file_containing_symbol: Option<String>,
    file_by_filename: Option<String>,
    list_services: Option<String>,
    file_containing_extension: Option<ReflectionExtensionRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReflectionExtensionRequest {
    containing_type: String,
    extension_number: i32,
}

pub fn build_list_services_request() -> Vec<u8> {
    // Field 3 (list_services): empty string
    encode_protobuf_list_services()
}

pub fn parse_list_services_response(data: &[u8]) -> Vec<GrpcService> {
    let mut services = Vec::new();

    // Parse the ListServiceResponse message: repeated ServiceResponse (field 1)
    let fields = super::parser::parse_protobuf_wire(data);
    for field in fields {
        if field.field_number == 1 {
            // Each ServiceResponse contains name (field 1)
            if let ProtobufValue::LengthDelimited(ref name_data) = field.value {
                if let Some(name) = extract_string_field(name_data, 1) {
                    let service = GrpcService {
                        name: name.clone(),
                        package: None,
                        methods: Vec::new(),
                        source: "reflection".to_string(),
                    };
                    services.push(service);
                }
            }
        }
    }

    services
}

fn extract_string_field(data: &[u8], field_num: u32) -> Option<String> {
    let fields = super::parser::parse_protobuf_wire(data);
    for field in fields {
        if field.field_number == field_num {
            if let ProtobufValue::LengthDelimited(bytes) = field.value {
                return String::from_utf8(bytes).ok();
            }
        }
    }
    None
}

fn encode_protobuf_list_services() -> Vec<u8> {
    // Field 3 (list_services): string value ""
    let mut buf = Vec::new();
    // tag = field_number << 3 | wire_type, field 3, wire_type 2
    let tag: u8 = (3 << 3) | 2;
    buf.push(tag);
    buf.push(0); // length = 0 (empty string)
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_list_services_request() {
        let req = build_list_services_request();
        // Should be: tag(0x1a), length(0x00) for field 3, empty string
        assert_eq!(req, vec![0x1a, 0x00]);
    }

    #[test]
    fn test_parse_list_services_empty() {
        let services = parse_list_services_response(&[]);
        assert!(services.is_empty());
    }
}
