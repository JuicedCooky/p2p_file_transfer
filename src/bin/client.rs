use std::io::Write;
// use tokio::io::AsyncWriteExt;
use std::net::TcpListener;
use std::net::TcpStream;

// use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    //notes for ip address of the current device/server:
    //->ipconfig, to list ip addresses
    //Wireless LAN adapter Wi-Fi -> IPv4 Address
    let mut ip_addr = "172.21.208.1:6142";
    
    print!("Enter ip-address:");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut ip_addr.to_string()).unwrap();
    
    let mut stream = TcpStream::connect(ip_addr);
    // println!("Connected to server {}",ip_addr);

    Ok(())
}