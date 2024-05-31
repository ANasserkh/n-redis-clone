use bytes::{Buf, Bytes};

pub fn encode_length(start: u8, bytes: &mut Bytes) -> u32 {
    match start >> 5 {
        0x00 => start as u32,
        0x01 => {
            let start = start & 0x1F;
            u32::from_be_bytes([0x00, 0x00, bytes.get_u8(), start])
        }
        0x02 => bytes.get_u32_le() as u32,
        0x03 => bytes.get_u64_le() as u32, // TODO: Handle string encoded length
        _ => 00,
    }
}

pub fn parse_key_value_pair(bytes: &mut Bytes) -> Result<(String, String), anyhow::Error> {
    let _type = bytes.get_u8();
    let length = encode_length(bytes.get_u8(), bytes);
    let key = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

    let length = encode_length(bytes.get_u8(), bytes);
    let value = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

    Ok((key, value))
}
