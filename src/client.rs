use std::error::Error;
use tokio::net::TcpStream;

use std::io::Write;
use super::utils;
use tokio::signal;
use std::sync::Arc;
use tokio::sync::Mutex;
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
            ip_addr = "10.160.6.186:6142".to_string();
            ip_addr = "10.160.18.168:6142".to_string();
        }
        else{
            ip_addr = ip_addr.trim().to_string();
        }
        let stream = TcpStream::connect(&ip_addr).await;
        match stream{
            Ok(_) =>{
                println!("Connected to Server: {}",ip_addr);
                let stream = Arc::new(Mutex::new(stream.unwrap())); 
                
                tokio::spawn(async {
                    loop{
                        // if (stream)
                    }
                    
                });
                let stream_cloned = Arc::clone(&stream);
                utils::display_options(stream_cloned).await;
                Ok(())
            }
            Err(e) => {
                println!("Failed to connect to Server: {}",ip_addr);
                Err(Box::new(e))
            }
        }
    }
}