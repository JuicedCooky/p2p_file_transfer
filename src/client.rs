// use tokio::io::AsyncWriteExt;
use std::net::TcpListener;
use std::net::TcpStream;

// use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let ip_addr = "172.21.208.1:6142";
    let mut stream = TcpStream::connect(ip_addr).await?;
    println!("Connected to server {}",ip_addr);

    Ok(())
}