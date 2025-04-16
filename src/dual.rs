use std::{error::Error, io::Write};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use local_ip_address::local_ip;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use rfd::FileDialog;
use std::path::PathBuf;
use tokio::time::{Duration, sleep};
use crate::thread::write::write_a_file_to_stream;
use crate::thread::write::write_a_folder_to_stream;
use std::sync::{Arc, Mutex as std_Mutex};
use crate::thread::read::{read_file_from_stream_dual, read_folder_from_stream_dual};
use std::fs::OpenOptions;

pub struct Dual{
    io_lock: Arc<std_Mutex<()>>
}

impl Dual {
    pub async fn new() -> Result<(), Box<dyn Error>> { 
        clearscreen::clear().expect("failed to clear screen");

        println!("Select file and folder reception locations, and log file folder");
        sleep(Duration::from_secs(2)).await;
      
        let file_save_location = match folder_init("Choose save location for files".to_string()).await {
            Some(path) => path,
            None => {
                println!("File folder selection cancelled or failed. Returning to menu.");
                return Ok(());
            }
        };

        let folder_save_location = match folder_init("Choose save location for folders".to_string()).await {
            Some(path) => path,
            None => {
                println!("File folder selection cancelled or failed. Returning to menu.");
                return Ok(());
            }
        };

        let log_save_location = match folder_init("Choose to save reception log".to_string()).await {
            Some(path) => path,
            None => {
                println!("File folder selection cancelled or failed. Returning to menu.");
                return Ok(());
            }
        };

        let log_path = log_save_location.join("transfer_log.txt");

        let dual = Arc::new(Dual {
            io_lock: Arc::new(std_Mutex::new(())),
        });

        // Clone for each task
        let dual_host = Arc::clone(&dual);
        let dual_client = Arc::clone(&dual);

        // Spawn both tasks
        let host_handle = tokio::spawn(async move {
            dual_host.host_sub_session(file_save_location, folder_save_location, log_path).await
        });

        // Pause to allow host subsession to obtain and print it's IP address
        sleep(Duration::from_secs(1)).await;

        let client_handle = tokio::spawn(async move {
            dual_client.client_sub_session().await
        });

        // Wait for both threads to complete
        let (host_result, client_result) = tokio::join!(host_handle, client_handle);

        println!("\nDual session ended. Host: {:?}, Client: {:?}", host_result, client_result);
        sleep(Duration::from_secs(3)).await;

        Ok(())
    }

    async fn host_sub_session(&self, file_save_location: PathBuf, folder_save_location: PathBuf, log_path: PathBuf) -> bool {

        // Bind to arbitrary port for receiving
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        match local_ip(){
            Ok(ip) => {
                {
                    let _lock = self.io_lock.lock().unwrap();
                    println!("\nNotice from host substream");
                    println!("Server running\nLocal Address: {}:{}", ip,port);
                    println!("Look to log file for details of received files and folders")
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
        let mut connect_stream: TcpStream;
        let connect_ip_addr: String;

        loop {
            log_to_file(&log_path, &format!("\nWaiting on connection"));
            
            // Initialize host stream
            let host_stream = listener.accept().await;

            // Gague connection success
            match host_stream {
                Ok((host_stream, addr)) => {
                    // Formalize Connection
                    connect_stream = host_stream;
                    connect_ip_addr = addr.ip().to_string();
                },
                Err(e) => {
                    log_to_file(&log_path, &format!("Failed connection :{}",e));
                    
                    break;
                },
            }

            // Inform client of acceptance
            let _ = connect_stream.write_all(b"Accepted\n").await;

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

                let _ = reader.read_line(&mut line).await;

                let send_type = line.trim().to_string();

                match send_type.as_str() {
                    "FILE" => {

                        // Initiate sending process for client 
                        let _ = lock.write_all(b"START FILE\n").await;

                        // Free lock for reading stream
                        drop(lock);

                        let stream_clone = Arc::clone(&connect_stream_copy);

                        let log_path_clone = log_path.clone();
                        read_file_from_stream_dual(stream_clone, file_save_location.clone(), log_path_clone).await;
                            
                    },
                    "FOLDER" => {
                            
                        // Initiate sending process for client 
                        let _ = lock.write_all(b"START FOLDER\n").await;
                            
                        // Free lock for reading stream
                        drop(lock);

                        let stream_clone = Arc::clone(&connect_stream_copy);

                        let log_path_clone = log_path.clone();
                        read_folder_from_stream_dual(stream_clone, connect_ip_addr.clone(), folder_save_location.clone(), 
                        log_path_clone).await;
                
                    },
                    "DISCONNECT" => {
                        {
                            let _lock = self.io_lock.lock().unwrap();
                            println!("\nNotice from host substream");
                            println!("\nClient disconnected");
                        }
                        break;
                    },
                    _ => {
                        {
                            let _lock = self.io_lock.lock().unwrap();
                            println!("\nNotice from host substream");
                            println!("Unexpected message, disconnecting from client");
                        }
                        break;
                    }

                }
            }

            // atomically access stdin stream
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nNotice from host substream");
                println!("Closing substream")
               
            }

            break;
        }

        // When returned, spawning function will know process is closed
        true
    }

    async fn client_sub_session(&self) -> bool{
    
        let connect_stream: Arc<tokio::sync::Mutex<TcpStream>>;
    
        loop {
            
            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nAction from client substream");
                print!("Enter ip-address:socket, or 'b' to close sub-stream. Note, substream cannot be reopened once closed:");
            }

            let ip_addr = tokio::task::spawn_blocking(|| take_input()).await.unwrap();

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
                            break;
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
                Err(_e) => { 

                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from client substream");
                        println!("Failed to connect to Server: {}",ip_addr);
                        print!("Enter 'c' to try again, other to close the sub-stream:");
                        std::io::stdout().flush().unwrap();          
                    }
                      
                    let cont_option = tokio::task::spawn_blocking(|| take_input()).await.unwrap();

                    match cont_option.as_str() {
                        "c" => continue,
                        _ => return true
                    }
                    
                }
            }

        }

        {
            let _lock = self.io_lock.lock().unwrap();
            println!("\nNotice from client substream");
            println!("Closing substream")
           
        }

        // When returned, spawning function will know process is closed
        true
    }

    async fn client_sub_session_handler(&self, connect_stream: Arc<tokio::sync::Mutex<TcpStream>>) -> bool{
        loop {
            // Initialize sender stream parameters
            let connect_stream_clone = Arc::clone(&connect_stream);
            let send_type: String;

            {
                let _lock = self.io_lock.lock().unwrap();
                println!("\nAction from client substream");
                println!("What would you like to do?");
                println!("1. Select File to send to host");
                println!("2. Select Folder to send to host");
                println!("3. Disconnect from host");
                print!("Enter choice:");
                std::io::stdout().flush().unwrap();             
            }
   
            let choice =  tokio::task::spawn_blocking(|| take_input()).await.unwrap();

            match choice.as_str(){
                "1" => send_type = String::from("FILE"),
                "2" => send_type = String::from("FOLDER"),
                "3" => {
                    // Acquire lock, and send disconnect message to host
                    let mut lock = connect_stream.lock().await;
                    let _ = lock.write_all(b"DISCONNECT\n").await;
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
                    let _ = lock.write_all(b"FILE\n").await;
                } else if send_type == "FOLDER" {
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nAction from client substream");
                        println!("\nSending type message ''FOLDER'' to host");
                    }
                    let _ = lock.write_all(b"FOLDER\n").await;
                }
            }

            // Receive file start message from host
            {   
                // Acquire lock
                let mut lock = connect_stream.lock().await;
                let mut reader = BufReader::new(&mut *lock);

                // Receive start message
                let mut init_message = String::new();

                let _ = reader.read_line(&mut init_message).await;

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
                    {
                        let _lock = self.io_lock.lock().unwrap();
                        println!("\nNotice from client substream");
                        println!("Unexpected message from server, breaking connection");
                    }
                    {
                        // Acquire lock, and send disconnect message to host
                        let mut lock = connect_stream.lock().await;
                        let _ = lock.write_all(b"DISCONNECT\n").await;
                    }
                    break;
                }

            }

        }

        true
    }
}

// Function to log messages on the host subsession
pub fn log_to_file(log_path: &PathBuf, message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        let _ = writeln!(file, "{}", message); // Fail silently
    }
}

// Helper function for taking input from the command line
pub fn take_input() -> String {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()  // 🔥 Trim and convert back to owned String
}

// Helper function for initializing folder paths
async fn folder_init(title: String) -> Option<PathBuf> {
    tokio::task::spawn_blocking(move || {
        FileDialog::new()
            .set_title(&title)
            .set_directory("/")
            .pick_folder()
    })
    .await
    .ok()
    .flatten()
}
