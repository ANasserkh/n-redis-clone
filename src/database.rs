use std::{collections::HashMap, io::Read};

use chrono::{DateTime, Utc};

pub struct Value {
    pub val: String,
    pub expire_at: Option<DateTime<Utc>>,
    pub r#type: String,
    pub entries: Option<HashMap<String, String>>,
}

pub struct Database {
    pub data: HashMap<String, Value>,
    pub config: HashMap<String, String>,
}

use crate::parser::Parser;
use anyhow::Result;
use bytes::Bytes;
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
        let mut byte_vec = vec![];
        fs::File::open(path)?.read_to_end(&mut byte_vec)?;
        let data = Parser::parse(Bytes::from(byte_vec))?;
        self.data = data;
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
