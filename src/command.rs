use std::collections::HashMap;

use crate::resp::encoder::simple_string_encode;
use anyhow::anyhow;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(
        &self,
        db: std::sync::MutexGuard<HashMap<String, String>>,
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

    fn set_command(&self, mut db: std::sync::MutexGuard<HashMap<String, String>>) -> String {
        let key = self.args[0].clone();
        let value = self.args[1].clone();
        db.insert(key, value);

        return String::from("+OK\r\n");
    }

    fn get_command(&self, db: std::sync::MutexGuard<HashMap<String, String>>) -> String {
        let value = db.get(&self.args[0]).unwrap();
        simple_string_encode(value)
    }
}
