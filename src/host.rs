use std::error::Error;


use tokio::{io::AsyncReadExt, net::TcpListener};
use local_ip_address::local_ip;

use super::utils;

pub struct Host{}

impl Host {
    pub async fn new(port: Option<&str>) -> Result<(), Box<dyn Error>>{
        let port = port.unwrap_or("6142");
        let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
        match local_ip(){
            Ok(ip) => println!("Server running\nLocal Address: {}:{}", ip,port),
            Err(e) => println!("Could not start server!\n{}",e.to_string()),
        }

        loop {
            let stream = listener.accept().await;
            while stream.iter().next().is_none(){
                println!("TEST");
            }
            println!("TEST");
            match stream{
                Ok((stream,addr)) => 
                {
                    println!("Connection from {}", addr);
                    //spawning a thread to handle options
                    tokio::spawn(async move{
                        utils::display_options(&stream).await;
                    });
                    // tokio::spawn(async move{
                    //     let mut buf = [0;10];
                    //     stream.read(&mut buf[..]);
                    // });
                }
                Err(e) => eprintln!("Failed connection :{}",e),
            }
        }
    }
}