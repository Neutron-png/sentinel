use super::errors::GrpcError;
use super::models::{GrpcFrame, ProtobufField, ProtobufValue, ProtobufWireType};

const FRAME_HEADER_SIZE: usize = 5;

pub fn parse_grpc_frames(data: &[u8]) -> Result<Vec<GrpcFrame>, GrpcError> {
    let mut frames = Vec::new();
    let mut offset = 0;

    while offset + FRAME_HEADER_SIZE <= data.len() {
        let compressed = data[offset] == 1;
        let length = u32::from_be_bytes([
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
        ]) as usize;

        offset += FRAME_HEADER_SIZE;

        if offset + length > data.len() {
            return Err(GrpcError::Parse(format!(
                "Frame length {} exceeds remaining data {} at offset {}",
                length,
                data.len() - offset,
                offset
            )));
        }

        let frame_data = data[offset..offset + length].to_vec();
        frames.push(GrpcFrame {
            compressed,
            length: length as u32,
            data: frame_data,
        });

        offset += length;
    }

    Ok(frames)
}

pub fn parse_protobuf_wire(data: &[u8]) -> Vec<ProtobufField> {
    let mut fields = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        if let Some((field, consumed)) = read_field(&data[offset..]) {
            if consumed == 0 {
                break;
            }
            fields.push(field);
            offset += consumed;
        } else {
            break;
        }
    }

    fields
}

fn read_field(data: &[u8]) -> Option<(ProtobufField, usize)> {
    let mut pos = 0;
    let (tag, tag_len) = read_varint(data)?;
    pos += tag_len;

    let field_number = (tag >> 3) as u32;
    let wire_type = ProtobufWireType::from_u32((tag & 0x07) as u32);

    let (value, val_len) = match wire_type {
        ProtobufWireType::Varint => {
            let (v, l) = read_varint(&data[pos..])?;
            (ProtobufValue::Varint(v), l)
        }
        ProtobufWireType::Fixed64 => {
            if pos + 8 > data.len() {
                return None;
            }
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[pos..pos + 8]);
            (ProtobufValue::Fixed64(u64::from_le_bytes(buf)), 8)
        }
        ProtobufWireType::LengthDelimited => {
            let (len, len_len) = read_varint(&data[pos..])?;
            let len = len as usize;
            let start = pos + len_len;
            if start + len > data.len() {
                return None;
            }
            (
                ProtobufValue::LengthDelimited(data[start..start + len].to_vec()),
                len_len + len,
            )
        }
        ProtobufWireType::Fixed32 => {
            if pos + 4 > data.len() {
                return None;
            }
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&data[pos..pos + 4]);
            (ProtobufValue::Fixed32(u32::from_le_bytes(buf)), 4)
        }
        _ => {
            if pos >= data.len() {
                return None;
            }
            (ProtobufValue::Unknown(data[pos..pos + 1].to_vec()), 1)
        }
    };
    pos += val_len;

    let field = ProtobufField {
        field_number,
        wire_type,
        name: None,
        value,
        path: vec![field_number],
        size_bytes: pos,
    };

    Some((field, pos))
}

fn read_varint(data: &[u8]) -> Option<(u64, usize)> {
    let mut value: u64 = 0;
    let mut shift = 0;
    let mut pos = 0;

    while pos < data.len() && pos < 10 {
        let byte = data[pos];
        pos += 1;
        value |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            return Some((value, pos));
        }
        shift += 7;
    }

    None
}

fn read_varint_u32(data: &[u8]) -> Option<(u32, usize)> {
    let (v, l) = read_varint(data)?;
    Some((v as u32, l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_grpc_frame() {
        let payload = b"hello";
        let mut data = vec![0u8]; // uncompressed
        data.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        data.extend_from_slice(payload);

        let frames = parse_grpc_frames(&data);
        assert!(frames.is_ok());
        let frames = frames.unwrap();
        assert_eq!(frames.len(), 1);
        assert!(!frames[0].compressed);
        assert_eq!(frames[0].data, payload);
    }

    #[test]
    fn test_parse_multiple_frames() {
        let p1 = b"hello";
        let p2 = b"world";
        let mut data = Vec::new();
        data.push(0u8);
        data.extend_from_slice(&(p1.len() as u32).to_be_bytes());
        data.extend_from_slice(p1);
        data.push(1u8); // compressed
        data.extend_from_slice(&(p2.len() as u32).to_be_bytes());
        data.extend_from_slice(p2);

        let frames = parse_grpc_frames(&data).unwrap();
        assert_eq!(frames.len(), 2);
        assert!(!frames[0].compressed);
        assert!(frames[1].compressed);
    }

    #[test]
    fn test_parse_protobuf_varint() {
        // Field 1, wire type 0 (varint), value 150
        let data = vec![0x08, 0x96, 0x01]; // tag=1(Wire0), value=150
        let fields = parse_protobuf_wire(&data);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].field_number, 1);
        assert_eq!(fields[0].wire_type, ProtobufWireType::Varint);
    }

    #[test]
    fn test_parse_protobuf_length_delimited() {
        // Field 2, wire type 2, value "test"
        let mut data = vec![0x12, 0x04]; // tag=2(Wire2), length=4
        data.extend_from_slice(b"test");
        let fields = parse_protobuf_wire(&data);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].field_number, 2);
        assert_eq!(fields[0].wire_type, ProtobufWireType::LengthDelimited);
    }

    #[test]
    fn test_parse_empty_grpc() {
        let frames = parse_grpc_frames(&[]).unwrap();
        assert!(frames.is_empty());
    }
}
