use crate::resp::encoder::simple_string_encode;
use anyhow::anyhow;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(&self) -> Result<String, anyhow::Error> {
        match self.name.to_lowercase().as_str() {
            "ping" => Ok(self.ping_command()),
            "echo" => Ok(self.echo_command()),
            _ => return Err(anyhow!("Command is not recognized {:?}", self.name)),
        }
    }

    pub fn ping_command(&self) -> String {
        "+PONG\r\n".to_string()
    }

    pub fn echo_command(&self) -> String {
        simple_string_encode(self.args.join(" "))
    }
}
