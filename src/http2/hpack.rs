#![allow(dead_code)]

use crate::http2::errors::Http2Error;

/// RFC 7541 HPACK codec.
///
/// Encodes with literal-without-indexing (0x10) representations and an
/// indexed-field representation for exact static-table matches. The dynamic
/// table is maintained for decode compatibility (insertions on
/// literal-with-incremental-indexing) but encode never adds entries, keeping
/// encode/decode round-trips deterministic.
#[derive(Debug, Clone)]
pub struct HpackContext {
    dynamic_table: Vec<HeaderField>,
    /// Current dynamic table size in bytes (entries + 32 each).
    dynamic_table_size: usize,
    /// Maximum dynamic table size in bytes.
    max_table_size: usize,
    /// Maximum header list size (octets of the decoded header list).
    max_header_list_size: u32,
}

#[derive(Debug, Clone)]
pub struct HeaderField {
    pub name: String,
    pub value: String,
}

impl HeaderField {
    fn size(&self) -> usize {
        self.name.len() + self.value.len() + 32
    }
}

impl HpackContext {
    pub fn new(max_header_list_size: u32) -> Self {
        Self {
            dynamic_table: Vec::new(),
            dynamic_table_size: 0,
            max_table_size: 4096,
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

    fn encode_header(
        &mut self,
        name: &str,
        value: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), Http2Error> {
        // Exact static match -> indexed field representation.
        if let Some(index) = static_table_index(name, value) {
            Self::encode_integer(index as u32, 7, 0x80, buf);
            return Ok(());
        }
        if let Some(index) = static_table_name_match(name) {
            // Literal-without-indexing: 4-bit prefix carries the name index.
            if index <= 15 {
                buf.push(0x00 | (index as u8));
                Self::encode_string(value, buf)?;
                return Ok(());
            }
        }
        // Literal name/value, never indexed (0x10 pattern, 4-bit prefix 0x0).
        buf.push(0x10);
        Self::encode_string(name, buf)?;
        Self::encode_string(value, buf)?;
        Ok(())
    }

    /// RFC 7541 §5.1 integer encoding with `prefix_bits` and an OR mask for
    /// the first byte's flag bits.
    pub fn encode_integer(value: u32, prefix_bits: u8, first_byte_flags: u8, buf: &mut Vec<u8>) {
        let max_prefix = (1u32 << prefix_bits) - 1;
        if value < max_prefix {
            buf.push(first_byte_flags | (value as u8));
        } else {
            buf.push(first_byte_flags | (max_prefix as u8));
            let mut remaining = value - max_prefix;
            while remaining >= 128 {
                buf.push((remaining % 128) as u8 | 0x80);
                remaining /= 128;
            }
            buf.push(remaining as u8);
        }
    }

    /// RFC 7541 §5.2 string literal encoding without Huffman coding.
    fn encode_string(s: &str, buf: &mut Vec<u8>) -> Result<(), Http2Error> {
        let bytes = s.as_bytes();
        if bytes.len() as u64 > u32::MAX as u64 {
            return Err(Http2Error::Compression("String too long".into()));
        }
        // H bit = 0 (no Huffman), 7-bit prefix length.
        Self::encode_integer(bytes.len() as u32, 7, 0x00, buf);
        buf.extend_from_slice(bytes);
        Ok(())
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<(String, String)>, Http2Error> {
        let mut headers = Vec::new();
        let mut header_list_octets = 0usize;
        let mut offset = 0;

        while offset < data.len() {
            let byte = data[offset];
            if byte & 0x80 != 0 {
                // 6.1 Indexed Header Field
                let index = Self::decode_integer(data, &mut offset, 7)?;
                let (name, value) = self.resolve_index(index)?;
                header_list_octets += name.len() + value.len() + 32;
                headers.push((name, value));
            } else if byte & 0xc0 == 0x40 {
                // 6.2.1 Literal with Incremental Indexing
                let index = Self::decode_integer(data, &mut offset, 6)?;
                let (name, value) = self.decode_literal_pair(data, &mut offset, index)?;
                self.insert_dynamic(&name, &value);
                header_list_octets += name.len() + value.len() + 32;
                headers.push((name, value));
            } else if byte & 0xe0 == 0x20 {
                // 6.3 Dynamic Table Size Update
                let size = Self::decode_integer(data, &mut offset, 5)?;
                self.set_dynamic_table_size(size)?;
            } else {
                // 6.2.2/6.2.3 Literal without indexing / never indexed
                let index = Self::decode_integer(data, &mut offset, 4)?;
                let (name, value) = self.decode_literal_pair(data, &mut offset, index)?;
                header_list_octets += name.len() + value.len() + 32;
                headers.push((name, value));
            }

            if self.max_header_list_size > 0
                && header_list_octets > self.max_header_list_size as usize
            {
                return Err(Http2Error::Compression(
                    "Header list exceeds maximum size".into(),
                ));
            }
        }
        Ok(headers)
    }

    fn decode_literal_pair(
        &self,
        data: &[u8],
        offset: &mut usize,
        index: u32,
    ) -> Result<(String, String), Http2Error> {
        let name = if index == 0 {
            Self::decode_string(data, offset)?
        } else {
            self.resolve_index(index)?.0
        };
        let value = Self::decode_string(data, offset)?;
        Ok((name, value))
    }

    /// RFC 7541 §5.2 string literal decoding (Huffman flag must be clear;
    /// Huffman-coded literals are rejected rather than mis-decoded).
    fn decode_string(data: &[u8], offset: &mut usize) -> Result<String, Http2Error> {
        if *offset >= data.len() {
            return Err(Http2Error::Compression("Truncated string literal".into()));
        }
        let huffman = data[*offset] & 0x80 != 0;
        let len = Self::decode_integer(data, offset, 7)? as usize;
        if len > data.len().saturating_sub(*offset) {
            return Err(Http2Error::Compression(
                "String literal exceeds buffer".into(),
            ));
        }
        let bytes = &data[*offset..*offset + len];
        *offset += len;
        if huffman {
            return Err(Http2Error::Compression(
                "Huffman-coded strings are not supported".into(),
            ));
        }
        String::from_utf8(bytes.to_vec())
            .map_err(|_| Http2Error::Compression("String literal is not valid UTF-8".into()))
    }

    /// RFC 7541 §5.1 integer decoding.
    pub fn decode_integer(
        data: &[u8],
        offset: &mut usize,
        prefix_bits: u8,
    ) -> Result<u32, Http2Error> {
        let max_prefix = (1u32 << prefix_bits) - 1;
        if *offset >= data.len() {
            return Err(Http2Error::Compression("Truncated integer".into()));
        }
        let mut value = (data[*offset] as u32) & max_prefix;
        *offset += 1;
        if value < max_prefix {
            return Ok(value);
        }
        let mut shift = 0;
        loop {
            if *offset >= data.len() {
                return Err(Http2Error::Compression("Truncated integer".into()));
            }
            let byte = data[*offset] as u32;
            *offset += 1;
            if shift >= 28 {
                return Err(Http2Error::Compression("Integer overflow".into()));
            }
            value += (byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
        }
    }

    fn set_dynamic_table_size(&mut self, size: u32) -> Result<(), Http2Error> {
        let size = size as usize;
        if size > self.max_table_size {
            return Err(Http2Error::Compression(
                "Dynamic table size update exceeds protocol maximum".into(),
            ));
        }
        while self.dynamic_table_size > size {
            if let Some(last) = self.dynamic_table.pop() {
                self.dynamic_table_size -= last.size();
            } else {
                break;
            }
        }
        Ok(())
    }

    fn insert_dynamic(&mut self, name: &str, value: &str) {
        let field = HeaderField {
            name: name.to_string(),
            value: value.to_string(),
        };
        let entry_size = field.size();
        if entry_size > self.max_table_size {
            // Entry larger than the table: empty the table and drop it.
            self.dynamic_table.clear();
            self.dynamic_table_size = 0;
            return;
        }
        self.dynamic_table.insert(0, field);
        self.dynamic_table_size += entry_size;
        while self.dynamic_table_size > self.max_table_size {
            if let Some(last) = self.dynamic_table.pop() {
                self.dynamic_table_size -= last.size();
            } else {
                break;
            }
        }
    }

    fn resolve_index(&self, index: u32) -> Result<(String, String), Http2Error> {
        if index == 0 {
            return Err(Http2Error::Compression("Invalid index 0".into()));
        }
        if let Some((name, value)) = STATIC_TABLE.get(index as usize - 1) {
            return Ok((name.to_string(), value.to_string()));
        }
        let dynamic_index = (index as usize) - STATIC_TABLE.len() - 1;
        if let Some(field) = self.dynamic_table.get(dynamic_index) {
            return Ok((field.name.clone(), field.value.clone()));
        }
        Err(Http2Error::Compression(format!("Index {index} not found")))
    }
}

impl Default for HpackContext {
    fn default() -> Self {
        Self::new(16777216)
    }
}

/// 1-based HPACK static table entry for an exact (name, value) match.
fn static_table_index(name: &str, value: &str) -> Option<usize> {
    STATIC_TABLE
        .iter()
        .position(|(n, v)| n.eq_ignore_ascii_case(name) && v.eq_ignore_ascii_case(value))
        .map(|p| p + 1)
}

/// 1-based static table index of the first entry whose name matches.
fn static_table_name_match(name: &str) -> Option<usize> {
    STATIC_TABLE
        .iter()
        .position(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|p| p + 1)
}

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
        assert_eq!(decoded, headers);
    }

    #[test]
    fn test_encode_decode_mixed_size() {
        let mut ctx = HpackContext::new(4096);
        let headers = vec![
            (":method".to_string(), "POST".to_string()),
            ("content-type".to_string(), "application/json".to_string()),
            (
                "x-long-header".to_string(),
                "a".repeat(500),
            ),
            ("x-empty".to_string(), String::new()),
        ];
        let encoded = ctx.encode(&headers).unwrap();
        let decoded = ctx.decode(&encoded).unwrap();
        assert_eq!(decoded, headers);
    }

    #[test]
    fn test_decodes_incremental_indexing_literals() {
        // Literal with incremental indexing (0x40), both name and value literal.
        let name = b"custom-header";
        let value = b"custom-value";
        let mut buf = Vec::new();
        buf.push(0x40); // 6-bit index prefix 0 -> literal name follows
        HpackContext::encode_integer(name.len() as u32, 7, 0x00, &mut buf);
        buf.extend_from_slice(name);
        HpackContext::encode_integer(value.len() as u32, 7, 0x00, &mut buf);
        buf.extend_from_slice(value);

        let mut ctx = HpackContext::new(4096);
        let decoded = ctx.decode(&buf).unwrap();
        assert_eq!(
            decoded,
            vec![(
                "custom-header".to_string(),
                "custom-value".to_string()
            )]
        );

        // The entry must now be resolvable from the dynamic table
        // (static table has 61 entries -> dynamic entry is index 62).
        let mut buf2 = Vec::new();
        HpackContext::encode_integer(62, 7, 0x80, &mut buf2);
        let decoded2 = ctx.decode(&buf2).unwrap();
        assert_eq!(decoded2, decoded);
    }

    #[test]
    fn test_rejects_huffman_and_truncation() {
        let mut ctx = HpackContext::new(4096);
        // Huffman-flagged string literal (H bit set) must be rejected, not garbled.
        assert!(ctx.decode(&[0x10, 0x83, b'a', b'b', b'c']).is_err());
        // Truncated literal must error, never panic.
        assert!(ctx.decode(&[0x10, 0x10]).is_err());
        // Index 0 is invalid.
        assert!(ctx.decode(&[0x80]).is_err());
    }

    #[test]
    fn test_header_list_size_limit() {
        let mut ctx = HpackContext::new(16);
        let headers = vec![(
            "x-oversized".to_string(),
            "0".repeat(100),
        )];
        let mut ctx_enc = HpackContext::new(4096);
        let encoded = ctx_enc.encode(&headers).unwrap();
        assert!(ctx.decode(&encoded).is_err());
    }
}
