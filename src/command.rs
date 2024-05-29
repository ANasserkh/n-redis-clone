use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

use crate::{
    resp::encoder::{null_bulk_string_encode, simple_string_encode},
    Value,
};
use anyhow::anyhow;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(
        &self,
        db: std::sync::MutexGuard<HashMap<String, Value>>,
    ) -> Result<String, anyhow::Error> {
        match self.name.to_lowercase().as_str() {
            "ping" => Ok(self.ping_command()),
            "echo" => Ok(self.echo_command()),
            "set" => Ok(self.set_command(db)),
            "get" => Ok(self.get_command(db)),
            _ => return Err(anyhow!("Command is not recognized {}", self.name)),
        }
    }

    pub fn ping_command(&self) -> String {
        "+PONG\r\n".to_string()
    }

    pub fn echo_command(&self) -> String {
        simple_string_encode(&self.args.join(" "))
    }

    fn set_command(&self, mut db: std::sync::MutexGuard<HashMap<String, Value>>) -> String {
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

        db.insert(
            key,
            Value {
                val: value,
                expire_at: expire_at,
            },
        );

        return String::from("+OK\r\n");
    }

    fn get_command(&self, db: std::sync::MutexGuard<HashMap<String, Value>>) -> String {
        let value = db.get(&self.args[0]);
        if value.is_none() {
            return null_bulk_string_encode();
        }

        let value = value.unwrap();

        if value.expire_at.is_none() {
            return simple_string_encode(&value.val);
        }

        if value.expire_at.unwrap() > Utc::now() {
            return simple_string_encode(&value.val);
        }

        return null_bulk_string_encode();
    }
}
