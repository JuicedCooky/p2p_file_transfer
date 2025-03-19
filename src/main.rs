// use get_if_addrs::Interface;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;


use tokio::io::AsyncReadExt;

use std::error::Error;
use std::io::Write;
use std::os::windows::raw::HANDLE;
use std::string;

// use std::fs;

// use rfd::FileDialog;

// use get_if_addrs::get_if_addrs;

mod host;
mod client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut choice = String::new();
    
    println!("1. Be host device.");
    println!("2. Be client to connect to host.");
    print!("Enter choice:");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut choice).unwrap();
    choice = String::from(choice.trim());

    if "1" == choice
    {
        
        let host = host::Host::new(Some("6142")).await?;
    }
    else if "2" == choice
    {
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

        if TcpStream::connect(&ip_addr).await.is_ok(){
            println!("Connected to Server: {}",ip_addr);
        }
        else{
            println!("Failed to connect to Server: {}",ip_addr);
        }
    }
    else {
        println!("ERROR NO CHOICE SPECIFIED");
    }



    Ok(())
}