#![allow(dead_code)]

use crate::http3::errors::Http3Error;

pub struct QpackContext {
    dynamic_table: Vec<(String, String)>,
    table_size: u32,
    max_table_size: u32,
}

impl QpackContext {
    pub fn new(max_table_size: u32) -> Self {
        Self {
            dynamic_table: Vec::with_capacity(64),
            table_size: 0,
            max_table_size,
        }
    }

    pub fn encode(&mut self, headers: &[(String, String)]) -> Result<Vec<u8>, Http3Error> {
        let mut buf = Vec::new();
        for (name, value) in headers {
            let n = name.as_bytes();
            let v = value.as_bytes();
            buf.push(0x00); // not indexed
            self.encode_length_prefixed(n, &mut buf);
            self.encode_length_prefixed(v, &mut buf);
        }
        Ok(buf)
    }

    fn encode_length_prefixed(&self, data: &[u8], buf: &mut Vec<u8>) {
        let len = data.len() as u32;
        self.encode_varint(len, buf);
        buf.extend_from_slice(data);
    }

    fn encode_varint(&self, value: u32, buf: &mut Vec<u8>) {
        if value < 64 {
            buf.push((value & 0x3F) as u8);
        } else if value < 16384 {
            buf.push((64 | ((value >> 8) & 0x3F)) as u8);
            buf.push((value & 0xFF) as u8);
        } else if value < 1073741824 {
            buf.push((128 | ((value >> 24) & 0x3F)) as u8);
            buf.push(((value >> 16) & 0xFF) as u8);
            buf.push(((value >> 8) & 0xFF) as u8);
            buf.push((value & 0xFF) as u8);
        } else {
            buf.push((192 | ((value >> 56) & 0x3F)) as u8);
            buf.push(((value >> 48) & 0xFF) as u8);
            buf.push(((value >> 40) & 0xFF) as u8);
            buf.push(((value >> 32) & 0xFF) as u8);
            buf.push(((value >> 24) & 0xFF) as u8);
            buf.push(((value >> 16) & 0xFF) as u8);
            buf.push(((value >> 8) & 0xFF) as u8);
            buf.push((value & 0xFF) as u8);
        }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, Http3Error> {
        let mut headers = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let _ = data[offset]; // encoded field
            offset += 1;
            let name_len = self.decode_varint(data, &mut offset)?;
            let name_end = offset + name_len as usize;
            let name = String::from_utf8_lossy(&data[offset..name_end]).to_string();
            offset = name_end;
            let value_len = self.decode_varint(data, &mut offset)?;
            let value_end = offset + value_len as usize;
            let value = String::from_utf8_lossy(&data[offset..value_end]).to_string();
            offset = value_end;
            headers.push((name, value));
        }
        Ok(headers)
    }

    fn decode_varint(&self, data: &[u8], offset: &mut usize) -> Result<u32, Http3Error> {
        let first = data[*offset];
        *offset += 1;
        let prefix = (first >> 6) & 0x03;
        match prefix {
            0 => Ok((first & 0x3F) as u32),
            1 => {
                if *offset >= data.len() {
                    return Err(Http3Error::Qpack("Truncated varint".into()));
                }
                let v = (((first & 0x3F) as u32) << 8) | (data[*offset] as u32);
                *offset += 1;
                Ok(v)
            }
            2 => {
                if *offset + 3 >= data.len() {
                    return Err(Http3Error::Qpack("Truncated varint".into()));
                }
                let mut v = ((first & 0x3F) as u32) << 24;
                v |= (data[*offset] as u32) << 16;
                v |= (data[*offset + 1] as u32) << 8;
                v |= data[*offset + 2] as u32;
                *offset += 3;
                Ok(v)
            }
            3 => {
                if *offset + 7 >= data.len() {
                    return Err(Http3Error::Qpack("Truncated varint".into()));
                }
                let mut v = ((first & 0x3F) as u64) << 56;
                for i in 0..7 {
                    v |= (data[*offset + i] as u64) << (48 - i * 8);
                }
                *offset += 7;
                Ok(v as u32)
            }
            _ => unreachable!(),
        }
    }
}

impl Default for QpackContext {
    fn default() -> Self {
        Self::new(4096)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_headers() {
        let mut ctx = QpackContext::new(4096);
        let headers = vec![
            (":method".into(), "GET".into()),
            (":path".into(), "/index.html".into()),
            (":authority".into(), "example.com".into()),
        ];
        let encoded = ctx.encode(&headers).unwrap();
        let decoded = ctx.decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0], (":method".to_string(), "GET".to_string()));
    }
}
