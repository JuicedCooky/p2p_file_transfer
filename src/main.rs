use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;

use tokio::io::AsyncReadExt;

use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {

    println!("TESTING SENDER");

    //run ncat -l 6142 (in wsl or linux)
    //127.0.0.1 is the ip address of the current device
    let listener = TcpListener::bind("0.0.0.0:6142").await?;
    println!("Server running");

    loop {
        match listener.accept().await {
            Ok((stream,addr)) => 
            {
                println!("Connection from {}", addr);
            }
            Err(e) => eprintln!("Failed connection :{}",e),
        }
    }
    // let mut stream = TcpStream::connect("192.168.31.255:6142").await?;
    // println!("created stream");


    // let mut line = String::new();
    // println!("Send:");

    // let b1 = std::io::stdin().read_line(&mut line).unwrap();

    // let result = stream.write_all(line.as_bytes()).await;

    // println!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}