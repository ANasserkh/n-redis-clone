use lazy_static::lazy_static;
use resp::decoder::decode;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    vec,
};

use clap::Parser;
use database::Database;
use thread_pool::ThreadPool;
mod command;
mod database;
mod parser;
mod thread_pool;
mod resp {
    pub mod decoder;
    pub mod encoder;
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    dir: Option<String>,

    #[arg(long)]
    dbfilename: Option<String>,
}

lazy_static! {
    static ref DB: Mutex<Database> = Mutex::new(Database::new());
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    handle_config(args, DB.lock().unwrap());
    {
        let mut db = DB.lock().unwrap();
        if let Some(path) = db.get_path() {
            let _ = db.restore(path.as_str());
        }
    }
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

    Ok(())
}

fn handle_config(args: Args, mut db: std::sync::MutexGuard<Database>) {
    if let Some(dir) = args.dir {
        db.config.insert("dir".to_string(), dir);
    }
    if let Some(dbfilename) = args.dbfilename {
        db.config.insert("dbfilename".to_string(), dbfilename);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = vec![0; 512];
    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                if len > 0 {
                    let req = String::from_utf8(buf[0..len].to_vec()).unwrap();

                    let _ = match handle_command(req, DB.lock().unwrap()) {
                        Ok(i) => stream.write_all(i.as_bytes()),
                        Err(err) => stream.write_all(err.to_string().as_bytes()),
                    };
                }
            }
            Err(_) => break,
        }
    }
}

fn handle_command(
    req: String,
    db: std::sync::MutexGuard<Database>,
) -> Result<String, anyhow::Error> {
    let cmd = decode(&req)?;
    cmd.execute(db)
}
