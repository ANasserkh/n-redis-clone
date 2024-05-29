pub fn simple_string_encode(txt: &String) -> String {
    format!("${}\r\n{}\r\n", txt.len(), txt)
}
