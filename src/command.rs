use std::collections::HashMap;
#[allow(unused)]
use std::sync::Mutex;

use chrono::{Duration, Utc};

use crate::{
    database::{Database, Value},
    resp::encoder::{
        array_string_encode, bulk_string_encode, null_bulk_string_encode, simple_string_encode,
    },
};
use anyhow::anyhow;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(
        &mut self,
        db: std::sync::MutexGuard<Database>,
    ) -> Result<String, anyhow::Error> {
        match self.name.to_lowercase().as_str() {
            "ping" => Ok(self.ping_command()),
            "echo" => Ok(self.echo_command()),
            "set" => Ok(self.set_command(db)),
            "get" => Ok(self.get_command(db)),
            "config" => Ok(self.config_command(db)),
            "keys" => Ok(self.keys_command(db)),
            "type" => Ok(self.type_command(db)),
            "xadd" => Ok(self.xadd_command(db)),
            _ => return Err(anyhow!("Command is not recognized {}", self.name)),
        }
    }

    pub fn ping_command(&self) -> String {
        "+PONG\r\n".to_string()
    }

    pub fn echo_command(&self) -> String {
        bulk_string_encode(&self.args.join(" "))
    }

    fn set_command(&self, mut db: std::sync::MutexGuard<Database>) -> String {
        let key = self.args[0].clone();
        let value = self.args[1].clone();
        let mut expire_at = None;

        if self.args.len() > 2 {
            let expire_type = self.args[2].to_lowercase();
            let expire_after = self.args[3].parse::<i64>().unwrap();
            expire_at = match expire_type.as_str() {
                "px" => Some(Utc::now() + Duration::milliseconds(expire_after.into())),
                "ex" => Some(Utc::now() + Duration::seconds(expire_after.into())),
                _ => None,
            };
        }

        db.data.insert(
            key,
            Value {
                val: value,
                expire_at: expire_at,
                r#type: String::from("string"),
                entries: None,
            },
        );

        return String::from("+OK\r\n");
    }

    fn get_command(&self, db: std::sync::MutexGuard<Database>) -> String {
        let value = db.data.get(&self.args[0]);
        if value.is_none() {
            return null_bulk_string_encode();
        }

        let value = value.unwrap();

        if value.expire_at.is_none() {
            return bulk_string_encode(&value.val);
        }

        if value.expire_at.unwrap() > Utc::now() {
            return bulk_string_encode(&value.val);
        }

        return null_bulk_string_encode();
    }

    fn config_command(&self, db: std::sync::MutexGuard<Database>) -> String {
        let key = &self.args[1];
        let value = db.config.get(key);

        match value {
            None => null_bulk_string_encode(),
            Some(v) => array_string_encode(vec![key, v]),
        }
    }

    fn keys_command(&self, db: std::sync::MutexGuard<Database>) -> String {
        let pattern = &self.args[0];
        if pattern == "*" {
            let keys = db.data.keys().collect::<Vec<&String>>();
            return array_string_encode(keys);
        }
        return null_bulk_string_encode();
    }

    fn type_command(&self, db: std::sync::MutexGuard<Database>) -> String {
        let key = &self.args[0];
        let value = db.data.get(key);

        match value {
            None => simple_string_encode(&String::from("none")),
            Some(v) => simple_string_encode(&v.r#type),
        }
    }

    fn xadd_command(&mut self, mut db: std::sync::MutexGuard<Database>) -> String {
        let mut iter = self.args.iter_mut();
        let stream_key = iter.next().unwrap().clone();
        let id = iter.next().unwrap();
        let mut entries = HashMap::new();

        while iter.len() > 0 {
            let key = iter.next().unwrap().clone();
            let value = iter.next().unwrap().clone();
            entries.insert(key, value);
        }

        let value = Value {
            r#type: String::from("stream"),
            expire_at: None,
            entries: Some(entries),
            val: id.clone(),
        };
        db.data.insert(stream_key, value);

        bulk_string_encode(id)
    }
}

#[cfg(test)]
mod commands_tests {
    use super::*;
    #[test]
    fn test_key_command() {
        let mut db = Database::new();
        db.data.insert(
            String::from("key1"),
            Value {
                expire_at: None,
                r#type: String::from("string"),
                val: String::from("value1"),
                entries: None,
            },
        );

        let mut cmd = Command {
            name: String::from("type"),
            args: vec![String::from("key1")],
        };

        let db: Mutex<Database> = Mutex::new(db);
        let db = db.lock().unwrap();
        let result = cmd.execute(db).unwrap();

        assert_eq!(result, simple_string_encode(&"string".to_string()))
    }

    #[test]
    fn test_key_command_missing_key() {
        let mut db = Database::new();
        db.data.insert(
            String::from("key1"),
            Value {
                expire_at: None,
                r#type: String::from("string"),
                val: String::from("value1"),
                entries: None,
            },
        );
        let mut cmd = Command {
            name: String::from("type"),
            args: vec![String::from("missing_key")],
        };

        let db: Mutex<Database> = Mutex::new(db);
        let db = db.lock().unwrap();
        let result = cmd.execute(db).unwrap();

        assert_eq!(result, simple_string_encode(&"none".to_string()))
    }

    #[test]
    fn test_xadd_command() {
        let db = Database::new();

        let mut cmd = Command {
            name: String::from("xadd"),
            args: vec![
                "stream_key".to_string(),
                "0-1".to_string(),
                "key".to_string(),
                "value".to_string(),
            ],
        };

        let db: Mutex<Database> = Mutex::new(db);

        {
            let db = db.lock().unwrap();
            let result = cmd.execute(db).unwrap();
            assert_eq!(result, bulk_string_encode(&"0-1".to_string()));
        }

        {
            let db = db.lock().unwrap();

            let mut cmd = Command {
                name: String::from("type"),
                args: vec![String::from("stream_key")],
            };

            let result = cmd.execute(db).unwrap();
            assert_eq!(result, simple_string_encode(&"stream".to_string()));
        }
    }
}
