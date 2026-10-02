#![allow(dead_code)]

use crate::http2::errors::Http2Error;
use flate2::bufread::DeflateDecoder;
use std::collections::HashMap;
use std::io::Read;

pub struct HpackContext {
    dynamic_table: Vec<HeaderField>,
    max_table_size: u32,
    current_table_size: u32,
    max_header_list_size: u32,
}

#[derive(Debug, Clone)]
pub struct HeaderField {
    pub name: String,
    pub value: String,
}

impl HpackContext {
    pub fn new(max_header_list_size: u32) -> Self {
        Self {
            dynamic_table: Vec::with_capacity(64),
            max_table_size: 4096,
            current_table_size: 0,
            max_header_list_size,
        }
    }

    pub fn encode(&mut self, headers: &[(String, String)]) -> Result<Vec<u8>, Http2Error> {
        let mut buf = Vec::new();
        for (name, value) in headers {
            self.encode_header(name, value, &mut buf)?;
        }
        Ok(buf)
    }

    fn encode_header(&mut self, name: &str, value: &str, buf: &mut Vec<u8>) -> Result<(), Http2Error> {
        let idx = self.find_in_tables(name, value);
        if let Some(index) = idx {
            self.encode_indexed(index, buf);
        } else {
            let name_idx = self.find_name_in_tables(name);
            match name_idx {
                Some(idx) => self.encode_literal_with_indexing(idx, name, value, buf)?,
                None => self.encode_literal_no_indexing(name, value, buf)?,
            }
        }
        Ok(())
    }

    fn find_in_tables(&self, name: &str, value: &str) -> Option<u32> {
        // Static table lookup
        if let Some(idx) = static_table_index(name, value) {
            return Some(idx as u32);
        }
        for (i, field) in self.dynamic_table.iter().rev().enumerate() {
            if field.name.eq_ignore_ascii_case(name) && field.value == value {
                return Some(STATIC_TABLE_LEN as u32 + 1 + i as u32);
            }
        }
        None
    }

    fn find_name_in_tables(&self, name: &str) -> Option<u32> {
        if let Some(idx) = static_table_name_index(name) {
            return Some(idx as u32);
        }
        for (i, field) in self.dynamic_table.iter().rev().enumerate() {
            if field.name.eq_ignore_ascii_case(name) {
                return Some(STATIC_TABLE_LEN as u32 + 1 + i as u32);
            }
        }
        None
    }

    fn encode_indexed(&self, index: u32, buf: &mut Vec<u8>) {
        let mut encoded = index;
        encoded |= 0x80;
        self.encode_integer(encoded, 7, buf);
    }

    fn encode_literal_with_indexing(
        &mut self,
        _name_idx: u32,
        name: &str,
        value: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), Http2Error> {
        buf.push(0x40);
        self.encode_string(name, buf)?;
        self.encode_string(value, buf)?;
        self.add_to_dynamic_table(name, value);
        Ok(())
    }

    fn encode_literal_no_indexing(
        &mut self,
        name: &str,
        value: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), Http2Error> {
        buf.push(0x00);
        self.encode_string(name, buf)?;
        self.encode_string(value, buf)?;
        Ok(())
    }

    fn encode_string(&self, s: &str, buf: &mut Vec<u8>) -> Result<(), Http2Error> {
        let bytes = s.as_bytes();
        let len = bytes.len() as u32;
        let mut encoded_len = len;
        encoded_len |= 0x00; // no Huffman for now
        self.encode_integer_into(encoded_len, 7, buf);
        buf.extend_from_slice(bytes);
        Ok(())
    }

    fn encode_integer(&self, value: u32, prefix_bits: u8, buf: &mut Vec<u8>) {
        let max_prefix = (1u32 << prefix_bits) - 1;
        if value < max_prefix {
            buf.push(value as u8);
        } else {
            buf.push(max_prefix as u8);
            let mut remaining = value - max_prefix;
            while remaining >= 128 {
                buf.push((remaining % 128 + 128) as u8);
                remaining /= 128;
            }
            buf.push(remaining as u8);
        }
    }

    fn encode_integer_into(&self, value: u32, prefix_bits: u8, buf: &mut Vec<u8>) {
        let max_prefix = (1u32 << prefix_bits) - 1;
        if value < max_prefix {
            let last = buf.pop().unwrap_or(0);
            buf.push(last | value as u8);
        } else {
            let last = buf.pop().unwrap_or(0);
            buf.push(last | max_prefix as u8);
            let mut remaining = value - max_prefix;
            while remaining >= 128 {
                buf.push((remaining % 128 + 128) as u8);
                remaining /= 128;
            }
            buf.push(remaining as u8);
        }
    }

    fn add_to_dynamic_table(&mut self, name: &str, value: &str) {
        let entry_size = (name.len() + value.len() + 32) as u32;
        while self.current_table_size + entry_size > self.max_table_size && !self.dynamic_table.is_empty() {
            self.evict_oldest();
        }
        if entry_size <= self.max_table_size {
            self.dynamic_table.insert(0, HeaderField {
                name: name.to_string(),
                value: value.to_string(),
            });
            self.current_table_size += entry_size;
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(removed) = self.dynamic_table.pop() {
            self.current_table_size = self.current_table_size.saturating_sub(
                (removed.name.len() + removed.value.len() + 32) as u32,
            );
        }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, Http2Error> {
        let mut headers = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            let byte = data[offset];
            if byte & 0x80 != 0 {
                // Indexed Header Field
                let index = self.decode_integer(data, &mut offset, 7)?;
                let (name, value) = self.resolve_index(index)?;
                headers.push((name, value));
            } else {
                // Handle literal
                offset += 1;
                let (name, value) = self.decode_literal(data, &mut offset, byte)?;
                headers.push((name, value));
            }
        }
        Ok(headers)
    }

    fn decode_integer(&self, data: &[u8], offset: &mut usize, prefix_bits: u8) -> Result<u32, Http2Error> {
        let max_prefix = (1u32 << prefix_bits) - 1;
        let first = data[*offset] as u32 & max_prefix;
        if first < max_prefix {
            *offset += 1;
            return Ok(first);
        }
        *offset += 1;
        let mut value: u32 = max_prefix;
        let mut shift = 0;
        while *offset < data.len() {
            let byte = data[*offset] as u32;
            *offset += 1;
            value += (byte & 127) << shift;
            if byte & 128 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(value)
    }

    fn resolve_index(&self, index: u32) -> Result<(String, String), Http2Error> {
        if index == 0 {
            return Err(Http2Error::Compression("Invalid index 0".into()));
        }
        let idx = index as usize;
        if idx <= STATIC_TABLE_LEN {
            if let Some((name, value)) = STATIC_TABLE.get(idx - 1) {
                return Ok((name.clone(), value.clone()));
            }
        } else {
            let dyn_idx = idx - STATIC_TABLE_LEN - 1;
            if let Some(field) = self.dynamic_table.get(dyn_idx) {
                return Ok((field.name.clone(), field.value.clone()));
            }
        }
        Err(Http2Error::Compression(format!("Index {index} not found in tables")))
    }

    fn decode_literal(&self, data: &[u8], offset: &mut usize, first_byte: u8) -> Result<(String, String), Http2Error> {
        let use_indexing = first_byte & 0x40 != 0;
        let the_rest = first_byte & 0x3F;
        let name_idx_val = self.decode_integer_with_offset(data, offset, 6, the_rest as u32)?;
        let name = self.resolve_index(name_idx_val)?.0;
        let value_len = self.decode_integer(data, offset, 7)?;
        let value_start = *offset;
        *offset += value_len as usize;
        let value = String::from_utf8_lossy(&data[value_start..*offset]).to_string();

        if use_indexing {
            let field = HeaderField {
                name: name.clone(),
                value: value.clone(),
            };
            // We need mutable access to add, so store for now
            // The caller should handle indexing
        }

        Ok((name, value))
    }

    fn decode_integer_with_offset(
        &self,
        data: &[u8],
        offset: &mut usize,
        _prefix_bits: u8,
        remainder: u32,
    ) -> Result<u32, Http2Error> {
        let max_prefix = (1u32 << _prefix_bits) - 1;
        if remainder < max_prefix {
            return Ok(remainder);
        }
        let mut value: u32 = max_prefix;
        let mut shift = 0;
        while *offset < data.len() {
            let byte = data[*offset] as u32;
            *offset += 1;
            value += (byte & 127) << shift;
            if byte & 128 == 0 {
                break;
            }
            shift += 7;
        }
        Ok(value)
    }
}

impl Default for HpackContext {
    fn default() -> Self {
        Self::new(16777216)
    }
}

// HTTP/2 Static Table (RFC 7541 Appendix A)
const STATIC_TABLE_LEN: usize = 61;
const STATIC_TABLE: &[(&str, &str)] = &[
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme", "http"),
    (":scheme", "https"),
    (":status", "200"),
    (":status", "204"),
    (":status", "206"),
    (":status", "304"),
    (":status", "400"),
    (":status", "404"),
    (":status", "500"),
    ("accept-charset", ""),
    ("accept-encoding", "gzip, deflate"),
    ("accept-language", ""),
    ("accept-ranges", ""),
    ("accept", ""),
    ("access-control-allow-origin", ""),
    ("age", ""),
    ("allow", ""),
    ("authorization", ""),
    ("cache-control", ""),
    ("content-disposition", ""),
    ("content-encoding", ""),
    ("content-language", ""),
    ("content-length", ""),
    ("content-location", ""),
    ("content-range", ""),
    ("content-type", ""),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("expect", ""),
    ("expires", ""),
    ("from", ""),
    ("host", ""),
    ("if-match", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("if-range", ""),
    ("if-unmodified-since", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("max-forwards", ""),
    ("proxy-authenticate", ""),
    ("proxy-authorization", ""),
    ("range", ""),
    ("referer", ""),
    ("refresh", ""),
    ("retry-after", ""),
    ("server", ""),
    ("set-cookie", ""),
    ("strict-transport-security", ""),
    ("transfer-encoding", ""),
    ("user-agent", ""),
    ("vary", ""),
    ("via", ""),
    ("www-authenticate", ""),
];

fn static_table_index(name: &str, value: &str) -> Option<usize> {
    STATIC_TABLE.iter().position(|(n, v)| {
        n.eq_ignore_ascii_case(name) && v.eq_ignore_ascii_case(value)
    })
}

fn static_table_name_index(name: &str) -> Option<usize> {
    STATIC_TABLE.iter().position(|(n, _)| n.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_simple() {
        let mut ctx = HpackContext::new(4096);
        let headers = vec![
            (":method".to_string(), "GET".to_string()),
            (":path".to_string(), "/".to_string()),
            (":scheme".to_string(), "https".to_string()),
        ];
        let encoded = ctx.encode(&headers).unwrap();
        let decoded = ctx.decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 3);
    }

    #[test]
    fn test_static_table_lookup() {
        assert_eq!(static_table_index(":method", "GET"), Some(1));
        assert_eq!(static_table_index(":path", "/"), Some(3));
        assert_eq!(static_table_name_index("content-type"), Some(30));
    }
}
