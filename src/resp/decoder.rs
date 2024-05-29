use crate::command::Command;

pub fn decode(command: &String) -> Result<Command, anyhow::Error> {
    let crlf = "\r\n";
    let rest = command.split("$").last().unwrap();
    let mut iter = rest.split(crlf).filter(|l| l.len() > 0);
    let _ = iter.next();
    let data = iter.collect::<Vec<&str>>();

    Ok(Command {
        name: data[0].to_string(),
        args: data[1..].iter().map(|s| s.to_string()).collect(),
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let result = decode(&"*1\r\n$3\r\nPING\r\n".to_string()).unwrap();

        assert_eq!(result.name, "PING");
    }

    #[test]
    fn test_decode2() {
        let result = decode(&"*1\r\n$3\r\nPING\r\nMessage\r\n".to_string()).unwrap();

        assert_eq!(result.name, "PING");
        assert_eq!(result.args[0], "Message");
    }
}
