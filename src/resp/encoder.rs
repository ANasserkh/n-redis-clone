pub fn simple_string_encode(txt: &String) -> String {
    format!("+{}\r\n", txt)
}

pub fn bulk_string_encode(txt: &String) -> String {
    format!("${}\r\n{}\r\n", txt.len(), txt)
}

pub fn array_string_encode(txt: Vec<&String>) -> String {
    let len = txt.len();
    let txt = txt
        .into_iter()
        .map(|f| bulk_string_encode(f))
        .collect::<String>();
    format!("*{}\r\n{}", len, txt)
}

pub fn null_bulk_string_encode() -> String {
    String::from("$-1\r\n")
}

#[allow(dead_code)]
pub fn null_encode() -> String {
    String::from("$-1\r\n")
}
