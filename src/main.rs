extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    println!("{}", buffer);
    ctx.set_contents(buffer).unwrap();
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8800")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
