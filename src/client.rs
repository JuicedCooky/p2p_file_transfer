// use tokio::io::AsyncWriteExt;
use std::net::TcpListener;

// use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

#[tokio::main]
pub async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("TESTING CLIENT");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}