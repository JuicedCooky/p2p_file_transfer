use std::{error::Error, fs::read, io::Write};


use bytes::buf;
use tokio::{io::{AsyncReadExt, Interest}, net::TcpListener};
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
                    println!("Connection from {}", addr);
                    let stream = Arc::new(Mutex::new(stream));
                    //spawning a thread to handle options
                    let stream_copy = Arc::clone(&stream);
                    //let options = tokio::spawn(async move{
                    //    utils::display_options(stream_copy).await;
                    //});
                    let stream_read_copy = Arc::clone(&stream);

                    let read_stream = tokio::spawn(async move{
                        // let mut buffer = String::new();
                        read_from_stream(stream_read_copy,addr.ip().to_string(),None).await;
                    });
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