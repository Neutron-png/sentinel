#![allow(dead_code)]

use crate::websocket::models::{WsDirection, WsFrame, WsOpcode};

pub fn parse_frame(
    data: &[u8],
    direction: WsDirection,
    connection_id: uuid::Uuid,
) -> Option<WsFrame> {
    if data.len() < 2 {
        return None;
    }
    let fin = (data[0] & 0x80) != 0;
    let opcode_byte = data[0] & 0x0F;
    let opcode = WsOpcode::from_u8(opcode_byte)?;
    let masked = (data[1] & 0x80) != 0;
    let mut payload_len = (data[1] & 0x7F) as u64;
    let mut offset = 2;

    if payload_len == 126 {
        if data.len() < 4 {
            return None;
        }
        payload_len = u16::from_be_bytes([data[2], data[3]]) as u64;
        offset = 4;
    } else if payload_len == 127 {
        if data.len() < 10 {
            return None;
        }
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(&data[2..10]);
        payload_len = u64::from_be_bytes(len_bytes);
        offset = 10;
    }

    let mut mask_key = [0u8; 4];
    if masked {
        if data.len() < offset + 4 {
            return None;
        }
        mask_key.copy_from_slice(&data[offset..offset + 4]);
        offset += 4;
    }

    let payload_end = offset + payload_len as usize;
    if data.len() < payload_end {
        return None;
    }
    let mut payload = data[offset..payload_end].to_vec();

    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask_key[i % 4];
        }
    }

    Some(WsFrame::new(
        connection_id,
        opcode,
        payload,
        direction,
        fin,
        masked,
    ))
}

pub fn build_frame(opcode: WsOpcode, payload: &[u8], fin: bool, masked: bool) -> Vec<u8> {
    let mut frame = Vec::new();
    let mut first_byte: u8 = opcode.to_u8();
    if fin {
        first_byte |= 0x80;
    }
    frame.push(first_byte);

    let len = payload.len();
    let mut second_byte: u8 = if masked { 0x80 } else { 0x00 };
    if len < 126 {
        second_byte |= len as u8;
        frame.push(second_byte);
    } else if len <= 65535 {
        second_byte |= 126;
        frame.push(second_byte);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        second_byte |= 127;
        frame.push(second_byte);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }

    if masked {
        let mask: [u8; 4] = rand::random();
        frame.extend_from_slice(&mask);
        frame.extend(payload.iter().enumerate().map(|(i, b)| b ^ mask[i % 4]));
    } else {
        frame.extend_from_slice(payload);
    }

    frame
}
