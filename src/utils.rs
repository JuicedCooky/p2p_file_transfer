// use std::io::Read;
use std::io::Write;
use tokio::sync::Mutex;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio::io;
use tokio::net::TcpStream;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use std::marker;
use std::pin::Pin;
use std::sync::Arc;

use rfd::FileDialog;
use bytes::{buf, BytesMut};

pub async fn display_options(stream: Arc<Mutex<TcpStream>>) -> (){
    // let mut file = File::create_new("");
    loop {
        let mut choice = String::new();
        println!("What would you like to do?");
        println!("1. Select File to send to host");
        print!("Enter choice:");
        
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice);
        choice = choice.trim().to_string();
        
        match choice.as_str(){
            "1" => {
                let cloned_stream = Arc::clone(&stream);
                file(cloned_stream).await;
            }
            _ => println!("Error: INVALID CHOICE."),
        }
    }
}
 
pub async fn file(stream: Arc<Mutex<TcpStream>>) -> (){
    let mut file_path = String::new();
    // let mut file = File::create_new("");
    let mut buffer = String::new();
    // }
    let file_path = FileDialog::new().set_directory("/".to_string()).pick_file();
    // let content = file.unwrap();
    let mut file_str = file_path.clone().unwrap();
    let mut file_str = file_str.to_str().unwrap();
    println!("test:{}",file_str);

    let mut content: Vec<u8> = Vec::new();
    match fs::read(file_str).await{
        Ok(result) => {content = result;},
        Err(e) => eprintln!("ERROR:{}",e),
    }
    // let mut file = File::open(file_str).await;
    // match file {
    //     Ok(mut file) => {
    //         println!("test:{}",file_str);
    //         let mut contents:Vec<u8> = vec![];
    //         file.read_to_end(&mut contents).await;
    //         for i in contents{
    //             println!("{}",i.to_string())
    //         }

    //     }
    //     Err(_) => {println!("error")}
    // }

    
    // let mut buffer = BytesMut::with_capacity(10);
    // let content = file.read  (&mut buffer).await;
    let mut content_str = String::from_utf8_lossy(&content);
    
    let mut stream_lock = stream.lock().await;

    stream_lock.write_all(&content).await;
    // stream_lock.write(b"test").await;

    println!("test:{}",content_str.as_ref());
}