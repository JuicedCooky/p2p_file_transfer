use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let file = FileDialog::new();
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.

    println!("TESTING SENDER");

    let mut stream = TcpStream::connect("192.168.0.711:8080").await?;
    println!("created stream");


    let mut line = String::new();
    println!("Send:");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let result = stream.write_all(line.as_bytes()).await;

    println!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}