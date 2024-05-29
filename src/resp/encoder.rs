pub fn simple_string_encode(txt: &String) -> String {
    format!("${}\r\n{}\r\n", txt.len(), txt)
}

pub fn null_bulk_string_encode() -> String {
    String::from("$-1\r\n")
}

pub fn null_encode() -> String {
    String::from("$-1\r\n")
}
