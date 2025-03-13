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
    // let mut ip_addr = "172.21.208.1:6142".to_string();
    let mut ip_addr = String::new();

    print!("Enter ip-address:");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut ip_addr).unwrap();

    if ip_addr.trim().is_empty(){
        println!("Using default ip-address.");
        ip_addr = "10.160.3.126:6142".to_string();
    }
    else{
        ip_addr = ip_addr.trim().to_string();
    }
    
    if let Ok(stream) = TcpStream::connect(&ip_addr){
        println!("Connected to Server: {}",ip_addr);
    }
    else{
        println!("Failed to connect to Server: {}",ip_addr);
    }

    Ok(())
}