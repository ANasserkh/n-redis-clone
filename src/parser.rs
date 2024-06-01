use std::collections::HashMap;

use anyhow::Ok;
use bytes::{Buf, Bytes};
use chrono::{DateTime, Utc};

use crate::database::Value;

pub struct Parser;

impl Parser {
    pub fn parse(mut bytes: Bytes) -> Result<HashMap<String, Value>, anyhow::Error> {
        bytes.advance(9);
        while bytes.get_u8() != 0xFE {}
        let _db_number = bytes.get_u8();
        let _resizable_felid = bytes.get_u8();

        let length_encode = Parser::encode_length(bytes.get_u8(), &mut bytes);
        let _expired_length_encode = Parser::encode_length(bytes.get_u8(), &mut bytes);

        let mut data = HashMap::new();
        for _ in 1..=length_encode {
            let expire_at;
            let value_type;
            let duration_type = bytes.get_u8();

            if duration_type == 0xFD || duration_type == 0xFC {
                expire_at = Parser::parse_expire_date(&duration_type, &mut bytes);
                value_type = bytes.get_u8();
            } else {
                expire_at = None;
                value_type = duration_type;
            }

            let (key, value) = Parser::parse_key_value_pair(&value_type, &mut bytes)?;
            data.insert(
                key,
                Value {
                    val: value,
                    expire_at,
                    r#type: String::from("string"),
                    entries: None,
                },
            );
        }
        Ok(data)
    }

    fn encode_length(start: u8, bytes: &mut Bytes) -> u32 {
        match start {
            0b00000000..=0b00111111 => u32::from_be_bytes([0x00, 0x00, 0x00, start]),
            0b01000000..=0b01111111 => u32::from_be_bytes([0x00, 0x00, bytes.get_u8(), start]),
            0b10000000..=0b10111111 => bytes.get_u32(),
            0b11000000..=0b11111111 => 00, // TODO: Handle string encoded length
        }
    }
    fn parse_key_value_pair(
        value_type: &u8,
        bytes: &mut Bytes,
    ) -> Result<(String, String), anyhow::Error> {
        let _ = value_type;

        let length = Parser::encode_length(bytes.get_u8(), bytes);
        let key = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

        let length = Parser::encode_length(bytes.get_u8(), bytes);
        let value = String::from_utf8(bytes.split_to(length as usize).to_vec())?;

        Ok((key, value))
    }

    fn parse_expire_date(duration_type: &u8, bytes: &mut Bytes) -> Option<DateTime<Utc>> {
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
}
