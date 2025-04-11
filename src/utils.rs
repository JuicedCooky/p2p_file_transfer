// use std::io::Read;
use std::io::Write;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use std::sync::Arc;
use crate::thread::write::{write_a_file, write_a_folder};

pub async fn display_options(stream: Arc<Mutex<TcpStream>>) -> (){
    // let mut file = File::create_new("");
    loop {
        let mut choice = String::new();
        println!("What would you like to do?");
        println!("1. Select File to send to host");
        println!("2. Select Folder to send to host");
        print!("Enter choice:");
        
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice);
        choice = choice.trim().to_string();
        
        match choice.as_str(){
            "1" => {
                let cloned_stream = Arc::clone(&stream);
                write_a_file(cloned_stream,None).await;
            }
            "2" => {
                let cloned_stream = Arc::clone(&stream);
                write_a_folder(cloned_stream).await;
            }
            _ => println!("Error: INVALID CHOICE."),
        }
    }
}
