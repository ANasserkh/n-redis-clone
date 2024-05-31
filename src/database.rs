use std::{collections::HashMap, io::Read};

use chrono::{DateTime, Utc};

pub struct Value {
    pub val: String,
    pub expire_at: Option<DateTime<Utc>>,
}

pub struct Database {
    pub data: HashMap<String, Value>,
    pub config: HashMap<String, String>,
}

use crate::parser::{encode_length, parse_expire_date, parse_key_value_pair};
use anyhow::Result;
use bytes::{Buf, Bytes};
use std::fs;
impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            config: HashMap::new(),
        }
    }

    pub fn get_path(&self) -> Option<String> {
        let dir = self.config.get("dir")?;
        let filename = self.config.get("dbfilename")?;
        Some(format!("{dir}/{filename}"))
    }

    pub fn restore(&mut self, path: &str) -> Result<(), anyhow::Error> {
        println!("path  = {}", path);
        let mut byte_vec = vec![];
        fs::File::open(path)?.read_to_end(&mut byte_vec)?;
        let mut bytes = Bytes::from(byte_vec);
        bytes.advance(9);

        while bytes.get_u8() != 0xFE {}
        let _db_number = bytes.get_u8();
        let _resizable_felid = bytes.get_u8();
        let length_encode = encode_length(bytes.get_u8(), &mut bytes);

        let expired_length_encode = encode_length(bytes.get_u8(), &mut bytes);

        if expired_length_encode != 0 {
            0xFD; // in second
            0xFC; // in millisecond
        }

        for _ in 1..=length_encode {
            let expire_at;
            let value_type;
            let duration_type = bytes.get_u8();

            if duration_type == 0xFD || duration_type == 0xFC {
                expire_at = parse_expire_date(&duration_type, &mut bytes);
                value_type = bytes.get_u8();
            } else {
                expire_at = None;
                value_type = duration_type;
            }

            let (key, value) = parse_key_value_pair(&value_type, &mut bytes)?;
            self.data.insert(
                key,
                Value {
                    val: value,
                    expire_at,
                },
            );
        }

        Ok(())
    }
}

#[test]
fn test_import() {
    let mut db = Database::new();
    let _result = db
        .restore("D:/Learning/codecrafters-redis-rust/src/temp/dump.rdb")
        .unwrap();
    assert_eq!(db.data.keys().len(), 3);
    let expired_keys = db.data.get("key_exp").unwrap();
    assert!(expired_keys.expire_at.is_some());
}
