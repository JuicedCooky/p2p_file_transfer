use std::{error::Error, fs::read, io::Write};


use bytes::buf;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, Interest}, net::TcpListener};
use local_ip_address::local_ip;

use super::utils;
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
use tokio::sync::Mutex;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use rfd::FileDialog;
use std::path::PathBuf;
use std::io::stdout;
use crate::thread::read::{self, read_file_from_stream, read_from_stream};

pub struct Host{}

impl Host {
    pub async fn new(port: Option<&str>) -> Result<(), Box<dyn Error>>{
        clearscreen::clear().expect("failed to clear screen");
        let port = port.unwrap_or("6142");
        let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).await?;
        match local_ip(){
            Ok(ip) => println!("Server running\nLocal Address: {}:{}", ip,port),
            Err(e) => println!("Could not start server!\n{}",e.to_string()),
        }

        loop {
            println!("Waiting on client");
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

                            let cont = handle_host_session(&stream, stream_read_copy, addr.ip().to_string()).await;

                            if cont {
                                continue;
                            } else {
                                return Ok(());
                            }
                            /*  
                            let read_stream = tokio::spawn(async move{
                                // let mut buffer = String::new();
                                read_from_stream(stream_read_copy,addr.ip().to_string(),None).await;
                            });
                            */
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

pub async fn handle_host_session(stream: &Arc<Mutex<TcpStream>>, stream_clone_base: Arc<Mutex<TcpStream>>, outgoing_adder:String) -> bool {

    let mut file_save_location = PathBuf::new();
        
    // Receive message of send_type to be sent
    loop {
        // Initialize outer lock and reader
        let mut lock = stream.lock().await;
        let mut reader = BufReader::new(&mut *lock);

        // Initialize line to be read from buffer
        let mut line = String::new();

        reader.read_line(&mut line).await;

        let send_type = line.trim().to_string();

        //println!("Received send_type message {} from client", send_type);

        match send_type.as_str() {
            "FILE" => {

                if file_save_location.as_mut_os_str().is_empty() {
                    file_save_location = FileDialog::new()
                    .set_title("Choose save location")
                    .set_directory("/"
                    .to_string())
                    .pick_folder()
                    .unwrap();
                }
                
                lock.write_all(b"START FILE\n").await;

                // Free lock for reading stream
                drop(lock);

                let stream_clone = Arc::clone(&stream_clone_base);
    
                read_file_from_stream(stream_clone, file_save_location.clone()).await;
                
            },
            "FOLDER" => {
                lock.write_all(b"START FOLDER\n").await;
                // initalize read procedure for folder
            },
            "DISCONNECT" => {
                println!("Client disconnected");
                break;
            },
            _ => {
                println!("Unexpected message, disconnecting from client");
                break;
            }

        }
    }

    let mut cont = String::new();

    println!("Would you like to continue as host? Enter 'y' for yes, other for no");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut cont).unwrap();

    if cont == "y" {
        return true;
    } else {
        return false;
    }

}