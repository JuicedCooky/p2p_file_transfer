// use std::io::Read;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use crate::dual;
use crate::thread::write::write_a_file_to_stream;
use crate::thread::write::write_a_folder_to_stream;

pub async fn display_options(stream: Arc<Mutex<TcpStream>>) -> (){
    // let mut file = File::create_new("");
    loop {
        println!("What would you like to do?");
        println!("1. Select File to send to host");
        println!("2. Select Folder to send to host");
        println!("3. Disconnect from host");
        print!("Enter choice:");
        
        let choice = dual::take_input();

        match choice.as_str(){
            "1" => {
                let cloned_stream = Arc::clone(&stream);
                let send_type = String::from("FILE");
                handle_sender_session(&stream, cloned_stream, send_type).await;
                //write_a_file(cloned_stream,None).await;
            }
            "2" => {
                let cloned_stream = Arc::clone(&stream);
                let send_type = String::from("FOLDER");
                handle_sender_session(&stream, cloned_stream, send_type).await;
                //write_a_folder(cloned_stream).await;
            }
            "3" => {
                // Acquire lock, and send disconnect message to host
                let mut lock = stream.lock().await;
                let _ = lock.write_all(b"DISCONNECT\n").await;
                return;
            }
            _ => println!("Error: INVALID CHOICE."),
        }
    }
}

pub async fn handle_sender_session(stream: &Arc<Mutex<TcpStream>>, stream_copy: Arc<Mutex<TcpStream>>,send_type: String) -> () {
 
    // Send file type to be sent to host
    {   
        // Acquire lock
        let mut lock = stream.lock().await;
        
        // Send message to host
        if send_type == "FILE" {
            println!("\nSending type message ''FILE'' to host");
            let _ = lock.write_all(b"FILE\n").await;
            //println!("Message sent");
        } else if send_type == "FOLDER" {
            println!("\nSending type message ''FOLDER'' to host");
            let _ = lock.write_all(b"FOLDER\n").await;
            // return;
        }
    }
   
    // Receive file start message from host
    {   
        // Acquire lock
        let mut lock = stream.lock().await;
        let mut reader = BufReader::new(&mut *lock);

        // Receive start message
        let mut init_message = String::new();

        let _ = reader.read_line(&mut init_message).await;

        let init_message = init_message.trim().to_string();

        if init_message == "START FILE" {
            // Free lock for writing stream
            drop(lock);
            write_a_file_to_stream(stream_copy, None, true).await;
            return;
        } else if init_message == "START FOLDER" {
            // Free lock for writing stream
            drop(lock);
            write_a_folder_to_stream(stream_copy,None,None).await;
            return;
        } else {
            return;
        }

    }
    
}