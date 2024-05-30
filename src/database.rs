use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct Value {
    pub val: String,
    pub expire_at: Option<DateTime<Utc>>,
}

pub struct Database {
    pub data: HashMap<String, Value>,
    pub config: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            config: HashMap::new(),
        }
    }
}
