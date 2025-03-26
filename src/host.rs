use std::{error::Error, fs::read, io::Write};


use bytes::buf;
use tokio::{io::{AsyncReadExt, Interest}, net::TcpListener};
use local_ip_address::local_ip;

use super::utils;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::stdout;


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
                    let options = tokio::spawn(async move{
                        utils::display_options(stream_copy).await;
                    });
                    let stream_read_copy = Arc::clone(&stream);

                    let read_stream = tokio::spawn(async move{
                        // let mut buffer = String::new();
                        loop{
                            let read_lock = stream_read_copy.lock().await;
                            match read_lock.ready(Interest::READABLE).await{
                                Ok(_) => {
                                    let mut buf =[0;4096];
                                    // if(read_lock.poll_read_ready(cx))
                                    match read_lock.try_read(&mut buf){
                                        Ok(n) => {
                                            println!("Read from {}:{}",addr,String::from_utf8_lossy(&buf));
                                        }
                                        // Ok(_) => {break;}
                                        Err(e) => { }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
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