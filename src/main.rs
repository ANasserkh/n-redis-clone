use resp::decoder::decode;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    vec,
};
use thread_pool::ThreadPool;
mod command;
mod thread_pool;
mod resp {
    pub mod decoder;
    pub mod encoder;
}
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = vec![0; 512];
    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                if len > 0 {
                    let req = String::from_utf8(buf[0..len].to_vec()).unwrap();
                    let _ = match handle_command(req) {
                        Ok(i) => stream.write_all(i.as_bytes()),
                        Err(err) => stream.write_all(err.to_string().as_bytes()),
                    };
                }
            }
            Err(_) => break,
        }
    }
}

fn handle_command(req: String) -> Result<String, anyhow::Error> {
    let cmd = decode(&req)?;
    cmd.execute()
}
