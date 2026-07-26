use super::models::{GrpcMessage, ProtobufField, ProtobufValue};

pub fn inspect_message_fields(data: &[u8]) -> Vec<ProtobufField> {
    super::parser::parse_protobuf_wire(data)
}

pub fn find_sensitive_fields(fields: &[ProtobufField]) -> Vec<&ProtobufField> {
    fields.iter().filter(|f| f.is_sensitive()).collect()
}

pub fn find_field_by_number(fields: &[ProtobufField], number: u32) -> Option<&ProtobufField> {
    fields.iter().find(|f| f.field_number == number)
}

pub fn estimate_message_size(fields: &[ProtobufField]) -> usize {
    fields.iter().map(|f| f.size_bytes).sum()
}

pub fn field_summary(field: &ProtobufField) -> String {
    let default_name = format!("field_{}", field.field_number);
    let name = field.name.as_deref().unwrap_or(&default_name);
    let wire = field.wire_type.label();
    let value = match &field.value {
        ProtobufValue::Varint(v) => v.to_string(),
        ProtobufValue::Fixed64(v) => format!("0x{:016x}", v),
        ProtobufValue::Fixed32(v) => format!("0x{:08x}", v),
        ProtobufValue::LengthDelimited(data) => {
            if let Ok(s) = String::from_utf8(data.clone()) {
                if s.len() > 64 {
                    format!("{}...", &s[..64])
                } else {
                    s
                }
            } else {
                format!("<{} bytes>", data.len())
            }
        }
        ProtobufValue::Unknown(data) => format!("<{} bytes>", data.len()),
    };
    format!("{} ({}) = {}", name, wire, value)
}

pub fn combined_frame_data(frames: &[super::models::GrpcFrame]) -> Vec<u8> {
    let mut data = Vec::new();
    for frame in frames {
        data.extend_from_slice(&frame.data);
    }
    data
}
