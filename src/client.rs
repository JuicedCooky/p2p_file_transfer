use std::error::Error;
use tokio::net::TcpStream;
use crate::dual;

use super::utils;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::{Duration, sleep};

pub struct Client{}

impl Client{
    pub async fn new() -> Result<(), Box<dyn Error>>{

        loop {

            clearscreen::clear().expect("failed to clear screen");

            // Choose ip-address to connect to
            print!("Enter ip-address:socket, or 'b' to go back to the menu:");
            
            let ip_addr = dual::take_input();

            if ip_addr == "b" {
                return Ok(());
            }

            let stream = TcpStream::connect(&ip_addr).await;

            match stream{
                
                Ok(mut stream) =>{

                    // Parse server's connection response
                    let mut reader = BufReader::new(&mut stream);
                    let mut host_response = String::new();
                    let bytes_read = reader.read_line(&mut host_response).await?;

                    if bytes_read == 0 {
                        println!("Server closed connection unexpectedly.");
                        continue;
                    }

                    host_response = host_response.trim().to_string();

                    match host_response.as_str() {
                        "Accepted" => {
                            println!("Server accepted the connection");
                            println!("Connected to Server: {}",ip_addr);

                            // Open menu of file and folder sharing options
                            let stream = Arc::new(Mutex::new(stream));
                            let stream_cloned = Arc::clone(&stream);
                            let options_join = tokio::spawn(async move{
                                utils::display_options(stream_cloned).await;
                            });
                            let _ = options_join.await;
                        },
                        "Rejected" => {
                            println!("Server rejected the connection");
                            sleep(Duration::from_secs(3)).await;
                            continue;
                        }
                        _ => {
                            println!("Unexpected response from server");
                            sleep(Duration::from_secs(3)).await;
                            continue;
                        }
                    }
             
                    return Ok(())
                }
                Err(_e) => {
                    println!("Failed to connect to Server: {}",ip_addr);

                    println!("Enter 'c' to try again, other to return to the menu");

                    let cont_option = dual::take_input();

                    match cont_option.as_str() {
                        "c" => continue,
                        _ => return Ok(())
                    }
                }
            }
        }
        
    }
}