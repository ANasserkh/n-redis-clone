use bytes::{Buf, Bytes};
use chrono::{DateTime, Utc};

pub fn encode_length(start: u8, bytes: &mut Bytes) -> u32 {
    match start {
        0b00000000..=0b00111111 => u32::from_be_bytes([0x00, 0x00, 0x00, start]),
        0b01000000..=0b01111111 => u32::from_be_bytes([0x00, 0x00, bytes.get_u8(), start]),
        0b10000000..=0b10111111 => bytes.get_u32(),
        0b11000000..=0b11111111 => 00, // TODO: Handle string encoded length
    }
}
pub fn parse_key_value_pair(
    value_type: &u8,
    bytes: &mut Bytes,
) -> Result<(String, String), anyhow::Error> {
    let _ = value_type;

    let length = encode_length(bytes.get_u8(), bytes);
    let key = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

    let length = encode_length(bytes.get_u8(), bytes);
    let value = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

    Ok((key, value))
}

pub fn parse_expire_date(duration_type: &u8, bytes: &mut Bytes) -> Option<DateTime<Utc>> {
    if duration_type == &0xFD {
        // duration in seconds
        let seconds = bytes.get_u32_le();
        return Some(DateTime::<Utc>::from_timestamp(seconds.into(), 0)?);
    }

    if duration_type == &0xFC {
        // duration in milliseconds
        let milliseconds: i64 = bytes.get_u64_le().try_into().unwrap();
        return Some(DateTime::<Utc>::from_timestamp_millis(milliseconds)?);
    }

    None
}
