use anyhow::anyhow;
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn execute(&self) -> Result<String, anyhow::Error> {
        match self.name.as_str() {
            "PING" => Ok(self.ping_command()),
            _ => return Err(anyhow!("Command is not recognized {:?}", self.name)),
        }
    }

    pub fn ping_command(&self) -> String {
        "+PONG\r\n".to_string()
    }
}
