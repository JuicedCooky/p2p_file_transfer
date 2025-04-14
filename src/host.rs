use std::{error::Error, fs::read, io::Write};


use bytes::buf;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, Interest}, net::TcpListener};
use local_ip_address::local_ip;

use super::utils;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::stdout;
use crate::thread::read::{self, read_from_stream};

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
            let mut stream = listener.accept().await;
            while stream.iter().next().is_none(){
                println!("TEST");
            }
            match stream{
                Ok((mut stream,addr)) => 
                {
                    println!("Requested connection from {}", addr);
                    println!("Enter 'y' to accept, other to reject");

                    let mut connect_option = String::new();

                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut connect_option).unwrap();

                    connect_option = String::from(connect_option.trim());

                    match connect_option.as_str() {
                        "y" => {

                            stream.write_all(b"Accepted\n").await?;
                            
                            let stream = Arc::new(Mutex::new(stream));
                   
                            let stream_read_copy = Arc::clone(&stream);

                            let read_stream = tokio::spawn(async move{
                                // let mut buffer = String::new();
                                read_from_stream(stream_read_copy,addr.ip().to_string(),None).await;
                            });
                        }
                        _=> {
                            stream.write_all(b"Rejected\n").await?;
                            stream.shutdown().await?;

                            drop(stream);
                            continue;
                        }
                    }

                    // let result = options.await;
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