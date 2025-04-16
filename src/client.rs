use std::error::Error;
use tokio::net::TcpStream;

use std::io::Write;
use super::utils;
use tokio::{signal, stream};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::{Duration, sleep};
use tokio::io::Interest;
pub struct Client{}
use crate::thread::read;

impl Client{
    pub async fn new() -> Result<(), Box<dyn Error>>{

        //notes for ip address of the current device/server:
        //->ipconfig, to list ip addresses
        //Wireless LAN adapter Wi-Fi -> IPv4 Address
        // let mut ip_addr = "172.21.208.1:6142".to_string();
        let mut ip_addr = String::new();

        loop {

            clearscreen::clear().expect("failed to clear screen");

            print!("Enter ip-address:socket, or 'b' to go back to the menu:");
            std::io::stdout().flush().unwrap();
            ip_addr.clear();
            std::io::stdin().read_line(&mut ip_addr).unwrap();

            ip_addr = ip_addr.trim().to_string();

            if ip_addr.is_empty() {
                println!("Using default ip-address.");
                ip_addr = "10.160.3.126:6142".to_string();
                ip_addr = "10.160.6.186:6142".to_string();
                ip_addr = "192.168.0.168:6142".to_string();
            }
            else if ip_addr == "b" {
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

                            let stream = Arc::new(Mutex::new(stream));
                            let stream_cloned = Arc::clone(&stream);
                            let options_join = tokio::spawn(async move{
                                utils::display_options(stream_cloned).await;
                            });
                            options_join.await;
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

                    
                    //let stream_read_copy = Arc::clone(&stream);  
                    // tokio::spawn(async move{
                        
                    // });
                    
                    return Ok(())
                }
                Err(e) => {
                    println!("Failed to connect to Server: {}",ip_addr);

                    let mut cont_option = String::new();

                    println!("Enter 'c' to try again, other to return to the menu");

                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut cont_option).unwrap();

                    cont_option = String::from(cont_option.trim());

                    match cont_option.as_str() {
                        "c" => continue,
                        _ => return Ok(())
                    }

                    //Err(Box::new(e))
                }
            }
        }
        
    }
}