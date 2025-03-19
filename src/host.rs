use std::error::Error;


use tokio::net::TcpListener;
use local_ip_address::local_ip;

pub struct Host{}

impl Host {
    pub async fn new(port: Option<&str>) -> Result<(), Box<dyn Error>>{
        let port = port.unwrap_or("6142");
        let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
        match local_ip(){
            Ok(ip) => println!("Server running\nLocal Address: {}", ip),
            Err(e) => println!("Could not start server!\n{}",e.to_string()),
        }

        loop {
            match listener.accept().await {
                Ok((stream,addr)) => 
                {
                    println!("Connection from {}", addr);
                }
                Err(e) => eprintln!("Failed connection :{}",e),
            }
        }
    }
}