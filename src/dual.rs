use std::{error::Error, fs::read, io::Write};
use bytes::buf;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, Interest}, net::TcpListener};
use local_ip_address::local_ip;
use super::utils;
use std::sync::Mutex;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use rfd::FileDialog;
use std::path::PathBuf;
use tokio::time::{Duration, sleep};
use crate::thread::write::write_a_file_to_stream;
use crate::thread::write::write_a_folder_to_stream;
use std::io::stdout;
use std::sync::{Arc, Mutex as std_Mutex};
use crate::thread::read::{self, read_file_from_stream, read_folder_from_stream, read_from_stream};

pub struct Dual{
    io_lock: Arc<std_Mutex<()>>
}

impl Dual {
    pub async fn new() -> Result<(), Box<dyn Error>> { 
        clearscreen::clear().expect("failed to clear screen");

        let dual = Arc::new(Dual {
            io_lock: Arc::new(std_Mutex::new(())),
        });

        // Clone for each task
        let dual_host = Arc::clone(&dual);
        let dual_client = Arc::clone(&dual);

        // Spawn both tasks
        let host_handle = tokio::spawn(async move {
            dual_host.host_sub_session().await
        });

        // Pause to allow host subsession to obtain and print it's IP address
        sleep(Duration::from_secs(1)).await;

        let client_handle = tokio::spawn(async move {
            dual_client.client_sub_session().await
        });

        // Wait for both to complete
        let (host_result, client_result) = tokio::join!(host_handle, client_handle);

        println!("\nDual session ended. Host: {:?}, Client: {:?}", host_result, client_result);
        sleep(Duration::from_secs(3)).await;

        Ok(())
    }

    async fn host_sub_session(&self) -> bool {

        // Bind to arbitrary port for receiving
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        match local_ip(){
            Ok(ip) => {
                {
                    let _lock = self.io_lock.lock().unwrap();
                    println!("\nNotice from host substream");
                    println!("Server running\nLocal Address: {}:{}", ip,port);
                }  
            },
            Err(e) => {
                {
                    let _lock = self.io_lock.lock().unwrap();
                    println!("\nNotice from host substream");
                    println!("Could not start server!\n{}",e.to_string());
                }
                return true;
            },
        }

        // Connect variables
        let mut connect_flag = false;
        let mut connect_stream: TcpStream;
        let mut connect_ip_addr: String;

        // File and folder save locations
        let mut file_save_location = PathBuf::new();
        let mut folder_save_location = PathBuf::new();

        loop {
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nNotice from host substream");
                println!("Waiting on connection");
            }
            
            // Initialize host stream
            let mut host_stream = listener.accept().await;

            // Make decision to proceed with connection
            match host_stream {
                Ok((mut host_stream, addr)) => {

                    connect_stream = host_stream;
                    connect_ip_addr = addr.ip().to_string();

                    let mut connect_option = String::new();

                    // atomically access stdin stream
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from host substream");
                        println!("Requested connection from {}", addr);
                        print!("Enter 'y' to accept, other to reject:");

                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut connect_option).unwrap();
                    } //drop lock

                    connect_option = String::from(connect_option.trim());
                
                    match connect_option.as_str() {
                        "y" => {
                            connect_flag = true;
                        },
                        _ => connect_flag = false
                    }

                },
                Err(e) => {
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nNotice from host-substream");
                        println!("Failed connection :{}",e);
                        
                    }
                    
                    break;
                },
            }

            if connect_flag {

                //println!("Enter reception branch")

                // Inform client of acceptance
                connect_stream.write_all(b"Accepted\n").await;

                // Initialize stream variables to pass to handler
                let connect_stream = Arc::new(tokio::sync::Mutex::new(connect_stream));
                let connect_stream_copy = Arc::clone(&connect_stream);

                // Enter file reception logic
                loop {
                    // Initialize outer lock and reader
                    let mut lock = connect_stream.lock().await;
                    let mut reader = BufReader::new(&mut *lock);

                    // Initialize line to be read from buffer
                    let mut line = String::new();

                    reader.read_line(&mut line).await;

                    let send_type = line.trim().to_string();

                    //println!("Received send_type message {} from client", send_type);

                    match send_type.as_str() {
                        "FILE" => {

                            if file_save_location.as_mut_os_str().is_empty() {
                                file_save_location = tokio::task::spawn_blocking(|| {
                                    FileDialog::new()
                                        .set_title("Choose save location")
                                        .set_directory("/".to_string())
                                        .pick_folder()
                                }).await.unwrap().unwrap_or_else(|| {
                                    //println!("User cancelled folder selection.");
                                    std::process::exit(0);
                                });
                            }
                            
                            lock.write_all(b"START FILE\n").await;

                            // Free lock for reading stream
                            drop(lock);

                            let stream_clone = Arc::clone(&connect_stream_copy);
                
                            read_file_from_stream(stream_clone, file_save_location.clone()).await;
                            
                        },
                        "FOLDER" => {
                            if folder_save_location.as_mut_os_str().is_empty() {
                                folder_save_location = tokio::task::spawn_blocking(|| {
                                    FileDialog::new()
                                        .set_title("Choose save location")
                                        .set_directory("/".to_string())
                                        .pick_folder()
                                }).await.unwrap().unwrap_or_else(|| {
                                    //println!("User cancelled folder selection.");
                                    std::process::exit(0);
                                });
                            } 

                            lock.write_all(b"START FOLDER\n").await;
                            
                            // Free lock for reading stream
                            drop(lock);

                            let stream_clone = Arc::clone(&connect_stream_copy);

                            read_folder_from_stream(stream_clone, connect_ip_addr.clone(), folder_save_location.clone()).await;
                
                        },
                        "DISCONNECT" => {
                            {
                                let _lock = self.io_lock.lock().unwrap();
                                println!("\nNotice from host-substream");
                                println!("\nClient disconnected");
                            }
                            break;
                        },
                        _ => {
                            {
                                let _lock = self.io_lock.lock().unwrap();
                                println!("\nNotice from host-substream");
                                println!("Unexpected message, disconnecting from client");
                            }
                            break;
                        }

                    }
                }

            } else {
                // Inform client of acceptance
                connect_stream.write_all(b"Rejected\n").await;
                connect_stream.shutdown().await;

                drop(connect_stream);
            }

            let mut cont = String::new();
            // atomically access stdin stream
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nAction from host substream");
                println!("Would you like to continue as host? Note that if you disconnect, you will have to start a new recession as host or dual to receive again");
                print!("Enter 'y' for yes, other for no:");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut cont).unwrap();
            }

            cont = String::from(cont.trim());

            if cont == "y" {
                continue;
            } else {
                break;
            }
        }

        // When returned, spawning function will know process is closed
        true
    }

    async fn client_sub_session(&self) -> bool{

        let mut ip_addr = String::new();
        let mut connect_stream: Arc<tokio::sync::Mutex<TcpStream>>;
    
        loop {
            // atomically access stdin stream
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nAction from client substream");
                print!("Enter ip-address:socket, or 'b' to close sub-stream. Note, substream cannot be reopened once closed:");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut ip_addr).unwrap();
            }

            ip_addr = ip_addr.trim().to_string();

            if ip_addr == "b" {
                return true;
            }
            
            // Attempt connection with stream
            let send_stream = TcpStream::connect(&ip_addr).await;

            match send_stream{ 
                Ok(mut send_stream) => {

                    // Parse server's connection response
                    let mut reader = BufReader::new(&mut send_stream);
                    let mut host_response = String::new();
                    let bytes_read = reader.read_line(&mut host_response).await.unwrap();

                    if bytes_read == 0 {
                        {
                            let _lock = self.io_lock.lock().unwrap();
                            println!("\nNotice from client substream");
                            println!("Server closed connection unexpectedly.");
                        }
                        continue;
                    }

                    host_response = host_response.trim().to_string();

                    match host_response.as_str() { 
                        "Accepted" => { 
                            {
                                let _lock = self.io_lock.lock().unwrap();
                                println!("\nNotice from client substream");
                                println!("Server accepted the connection");
                                println!("Connected to Server: {}",ip_addr);
                            }
                            
                            connect_stream = Arc::new(tokio::sync::Mutex::new(send_stream));

                            let _sub_result = self.client_sub_session_handler(connect_stream).await;
                            continue;
                        },
                        "Rejected" => {
                            {
                                let _lock = self.io_lock.lock().unwrap();
                                println!("\nNotice from client substream");
                                println!("Server rejected the connection");
                            }       
                            sleep(Duration::from_secs(3)).await;
                            continue;
                        },
                        _ => {
                            {
                                let _lock = self.io_lock.lock().unwrap();
                                println!("\nNotice from client substream");
                                println!("Unexpected response from server");
                            }   
                            sleep(Duration::from_secs(3)).await;
                            continue;
                        }
                    }
                
                },
                Err(e) => { 
                    let mut cont_option = String::new();

                    // atomically access stdin stream
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from client substream");
                        println!("Failed to connect to Server: {}",ip_addr);
                        print!("Enter 'c' to try again, other to close the sub-stream:");
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut cont_option).unwrap();
                    }

                    cont_option = String::from(cont_option.trim());

                    match cont_option.as_str() {
                        "c" => continue,
                        _ => return true
                    }
                    
                }
            }

        }

        // When returned, spawning function will know process is closed
        true
    }

    async fn client_sub_session_handler(&self, connect_stream: Arc<tokio::sync::Mutex<TcpStream>>) -> (){
        loop {
            // Initialize sender stream parameters
            let connect_stream_clone = Arc::clone(&connect_stream);
            let mut send_type = String::new();

            let mut choice = String::new();

            // atomically access stdin stream
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nAction from client substream");
                println!("What would you like to do?");
                println!("1. Select File to send to host");
                println!("2. Select Folder to send to host");
                println!("3. Disconnect from host");
                print!("Enter choice:");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut choice).unwrap();
            }

            choice = choice.trim().to_string();

            match choice.as_str(){
                "1" => send_type = String::from("FILE"),
                "2" => send_type = String::from("FOLDER"),
                "3" => {
                    // Acquire lock, and send disconnect message to host
                    let mut lock = connect_stream.lock().await;
                    lock.write_all(b"DISCONNECT\n").await;
                    break;
                }
                _ => continue
            }

            // Send file type to be sent to host
            {   
                // Acquire lock
                let mut lock = connect_stream.lock().await;
                
                // Send message to host
                if send_type == "FILE" {
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from client substream");
                        println!("\nSending type message ''FILE'' to host");
                    }
                    lock.write_all(b"FILE\n").await;
                    
                    //println!("Message sent");
                } else if send_type == "FOLDER" {
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from client substream");
                        println!("\nSending type message ''FOLDER'' to host");
                    }
                    lock.write_all(b"FOLDER\n").await;
                    // return;
                }
            }

            // Receive file start message from host
            {   
                // Acquire lock
                let mut lock = connect_stream.lock().await;
                let mut reader = BufReader::new(&mut *lock);

                // Receive start message
                let mut init_message = String::new();

                reader.read_line(&mut init_message).await;

                let init_message = init_message.trim().to_string();

                if init_message == "START FILE" {
                    // Free lock for writing stream
                    drop(lock);
                    write_a_file_to_stream(connect_stream_clone, None, true).await;
                    continue;
                } else if init_message == "START FOLDER" {
                    // Free lock for writing stream
                    drop(lock);
                    write_a_folder_to_stream(connect_stream_clone,None,None).await;
                    continue;
                } else {
                    continue;
                }

            }

        }

    }
}