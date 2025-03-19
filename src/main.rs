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
        let client = client::Client::new().await?;
    }
    else {
        println!("ERROR NO CHOICE SPECIFIED");
    }



    Ok(())
}