use std::error::Error;
use tokio::net::TcpStream;

use std::io::Write;
use super::utils;
pub struct Client{}

impl Client{
    pub async fn new() -> Result<(), Box<dyn Error>>{
            //notes for ip address of the current device/server:
        //->ipconfig, to list ip addresses
        //Wireless LAN adapter Wi-Fi -> IPv4 Address
        // let mut ip_addr = "172.21.208.1:6142".to_string();
        let mut ip_addr = String::new();

        print!("Enter ip-address:socket:");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut ip_addr).unwrap();

        if ip_addr.trim().is_empty(){
            println!("Using default ip-address.");
            ip_addr = "10.160.3.126:6142".to_string();
        }
        else{
            ip_addr = ip_addr.trim().to_string();
        }
        let stream = TcpStream::connect(&ip_addr).await;
        match stream{
            Ok(_) =>{
                println!("Connected to Server: {}",ip_addr);
                utils::display_options(&stream.unwrap());
                Ok(())
            }
            Err(e) => {
                println!("Failed to connect to Server: {}",ip_addr);
                Err(Box::new(e))
            }
        }
    }
}