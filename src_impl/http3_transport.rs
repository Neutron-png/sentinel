#![allow(dead_code)]

use rand::RngCore;

pub struct QuicTransport {
    version: u32,
}

impl QuicTransport {
    pub fn new() -> Self {
        Self { version: 1 }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn build_initial_packet(
        &self,
        dest_conn_id: &[u8],
        server_name: &str,
        quic_version: u32,
        max_data: u64,
        max_stream_data_bidi_local: u64,
        idle_timeout_ms: u64,
    ) -> Vec<u8> {
        let mut packet = Vec::new();

        // QUIC Long Header (Type 0x00 for Initial)
        packet.push(0xC0); // Header form(1) | Fixed bit(1) | Long Packet Type(2): Initial(00) | Reserved(2) | Packet Number Length(2): 4 bytes
        packet.extend_from_slice(&quic_version.to_be_bytes());

        // Destination Connection ID Length + ID
        packet.push(dest_conn_id.len() as u8);
        packet.extend_from_slice(dest_conn_id);

        // Source Connection ID Length + ID (random)
        let mut src_conn_id = vec![0u8; 8];
        rand::thread_rng().fill_bytes(&mut src_conn_id);
        packet.push(src_conn_id.len() as u8);
        packet.extend_from_slice(&src_conn_id);

        // Token Length (0 for initial)
        packet.push(0);

        // Payload length (placeholder - will be filled later)
        let payload_len_pos = packet.len();
        packet.extend_from_slice(&[0, 0]); // VarInt placeholder

        // Packet Number (4 bytes)
        let pn = [0x00, 0x00, 0x00, 0x01];
        packet.extend_from_slice(&pn);

        // Crypto frame
        let crypto_start = packet.len();
        packet.push(0x06); // CRYPTO frame type
        packet.push(0x00); // Offset = 0 (varint)

        // TLS ClientHello (simplified)
        let client_hello = Self::build_minimal_client_hello(server_name);
        let client_hello_len = client_hello.len() as u64;
        Self::write_varint(client_hello_len, &mut packet);
        packet.extend_from_slice(&client_hello);

        // QUIC Transport Parameters (simplified)
        let tp_start = packet.len();
        // max_idle_timeout (0x0001)
        packet.push(0x00);
        packet.push(0x01);
        Self::write_varint(idle_timeout_ms, &mut packet);

        // max_udp_payload_size (0x0003)  
        packet.push(0x00);
        packet.push(0x03);
        Self::write_varint(65527, &mut packet);

        // initial_max_data (0x0004)
        packet.push(0x00);
        packet.push(0x04);
        Self::write_varint(max_data, &mut packet);

        // initial_max_stream_data_bidi_local (0x0005)
        packet.push(0x00);
        packet.push(0x05);
        Self::write_varint(max_stream_data_bidi_local, &mut packet);

        // initial_max_streams_bidi (0x0008)
        packet.push(0x00);
        packet.push(0x08);
        Self::write_varint(100, &mut packet);

        // Now update the crypto frame length
        let crypto_data_len = (packet.len() - crypto_start - 1) as u64 - Self::varint_size(client_hello_len);
        // Fix crypto frame: insert length after frame type
        let len_byte_count = Self::varint_size(crypto_data_len);
        let old_frame_start = crypto_start;
        let _frame_type = packet.remove(old_frame_start);
        let _old_offset = packet.remove(old_frame_start);
        packet.insert(old_frame_start, 0x00);
        packet.insert(old_frame_start + 1, 0x06);
        Self::write_varint_at(crypto_data_len, &mut packet, old_frame_start + 2);

        // Update payload length
        let actual_payload = packet.len() - (payload_len_pos + 2) + 2; // Include PN
        let len_bytes = Self::varint_size(actual_payload as u64);
        packet.drain(payload_len_pos..payload_len_pos + 2);
        let mut len_buf = Vec::new();
        Self::write_varint(actual_payload as u64, &mut len_buf);
        for _i in 0..len_bytes {
            packet.insert(payload_len_pos, len_buf.pop().unwrap_or(0));
        }

        packet
    }

    fn build_minimal_client_hello(server_name: &str) -> Vec<u8> {
        // Minimal TLS 1.3 ClientHello structure
        let mut hello = Vec::new();
        // Handshake type: ClientHello (0x01)
        hello.push(0x01);
        // Length placeholder (3 bytes)
        let len_pos = hello.len();
        hello.extend_from_slice(&[0x00, 0x00, 0x00]);
        // TLS version: 1.2 (for compatibility in record)
        hello.extend_from_slice(&[0x03, 0x03]);
        // Random (32 bytes)
        let mut random = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut random);
        hello.extend_from_slice(&random);
        // Session ID (empty)
        hello.push(0x00);
        // Cipher suites: TLS_AES_128_GCM_SHA256 (0x1301) + TLS_AES_256_GCM_SHA384 (0x1302)
        hello.push(0x00);
        hello.push(0x04); // 4 bytes of cipher suites
        hello.extend_from_slice(&[0x13, 0x01, 0x13, 0x02]);
        // Compression methods: null (0x00)
        hello.push(0x01);
        hello.push(0x00);
        // Extensions
        let ext_start = hello.len();
        // SNI extension
        hello.extend_from_slice(&[0x00, 0x00]); // server_name type
        let sni_len_pos = hello.len();
        hello.extend_from_slice(&[0x00, 0x00]); // length placeholder
        let sni_data_start = hello.len();
        let name_bytes = server_name.as_bytes();
        hello.extend_from_slice(&[0x00]); // host_name type
        hello.extend_from_slice(&((name_bytes.len() as u16).to_be_bytes()));
        hello.extend_from_slice(name_bytes);
        let sni_data_len = (hello.len() - sni_data_start) as u16;
        hello[sni_len_pos] = (sni_data_len >> 8) as u8;
        hello[sni_len_pos + 1] = (sni_data_len & 0xFF) as u8;

        // Supported versions: TLS 1.3 (0x0304)
        hello.extend_from_slice(&[0x00, 0x2b]); // supported_versions type
        hello.extend_from_slice(&[0x00, 0x03]); // length
        hello.push(0x02); // length of versions
        hello.extend_from_slice(&[0x03, 0x04]);

        // Key share
        hello.extend_from_slice(&[0x00, 0x33]); // key_share type
        hello.extend_from_slice(&[0x00, 0x26]); // length
        hello.extend_from_slice(&[0x00, 0x24]); // client_shares length
        hello.extend_from_slice(&[0x00, 0x1d]); // group: x25519
        hello.extend_from_slice(&[0x00, 0x20]); // key_exchange length
        let mut key_share = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_share);
        hello.extend_from_slice(&key_share);

        // Extension length
        let ext_len = (hello.len() - ext_start) as u16;
        // Insert extension length after SNI
        let sni_ext_end = sni_data_start + sni_data_len as usize;
        let ext_len_start = sni_ext_end + 2; // Supported versions starts
        hello.insert(ext_len_start, (ext_len >> 8) as u8);
        hello.insert(ext_len_start + 1, (ext_len & 0xFF) as u8);

        // Fix ClientHello length
        let ch_len = (hello.len() - len_pos - 3) as u32;
        hello[len_pos] = (ch_len >> 16) as u8;
        hello[len_pos + 1] = ((ch_len >> 8) & 0xFF) as u8;
        hello[len_pos + 2] = (ch_len & 0xFF) as u8;

        hello
    }

    fn write_varint(value: u64, buf: &mut Vec<u8>) {
        let size = Self::varint_size(value);
        match size {
            1 => buf.push(value as u8),
            2 => buf.extend_from_slice(&((value as u16).to_be_bytes())),
            4 => buf.extend_from_slice(&((value as u32).to_be_bytes())),
            8 => buf.extend_from_slice(&(value.to_be_bytes())),
            _ => {}
        }
    }

    fn write_varint_at(value: u64, buf: &mut Vec<u8>, pos: usize) {
        let size = Self::varint_size(value);
        // Ensure we don't overflow
        if pos + size > buf.len() {
            buf.resize(pos + size, 0);
        }
        match size {
            1 => buf[pos] = value as u8,
            2 => {
                let bytes = (value as u16).to_be_bytes();
                buf[pos] = bytes[0];
                buf[pos + 1] = bytes[1];
            }
            4 => {
                let bytes = (value as u32).to_be_bytes();
                buf[pos..pos + 4].copy_from_slice(&bytes);
            }
            8 => {
                buf[pos..pos + 8].copy_from_slice(&value.to_be_bytes());
            }
            _ => {}
        }
    }

    fn varint_size(value: u64) -> usize {
        if value < 64 { 1 }
        else if value < 16384 { 2 }
        else if value < 1073741824 { 4 }
        else { 8 }
    }

    pub fn parse_version_negotiation(&self, data: &[u8]) -> Option<Vec<u32>> {
        if data.len() < 5 {
            return None;
        }
        let first = data[0];
        if (first & 0x80) == 0 {
            return None; // Short header
        }
        let _unused = data[0] >> 4; // Unused bits  
        if _unused != 0 {
            return None;
        }
        // Random source/dest connection IDs
        let versions_start = 5;
        let mut versions = Vec::new();
        let mut pos = versions_start;
        while pos + 4 <= data.len() {
            let v = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
            versions.push(v);
            pos += 4;
        }
        Some(versions)
    }
}

impl Default for QuicTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_initial_packet() {
        let transport = QuicTransport::new();
        let packet = transport.build_initial_packet(
            b"\x01\x02\x03\x04\x05\x06\x07\x08",
            "example.com",
            1,
            1048576,
            1048576,
            30000,
        );
        assert!(!packet.is_empty());
        assert_eq!(packet[0] & 0x80, 0x80); // Header form bit set
    }

    #[test]
    fn test_varint_size() {
        assert_eq!(QuicTransport::varint_size(0), 1);
        assert_eq!(QuicTransport::varint_size(63), 1);
        assert_eq!(QuicTransport::varint_size(64), 2);
        assert_eq!(QuicTransport::varint_size(16383), 2);
        assert_eq!(QuicTransport::varint_size(16384), 4);
    }
}
