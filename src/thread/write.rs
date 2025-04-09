use core::num;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use rfd::FileDialog;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::main;


pub async fn write_a_file(stream: Arc<Mutex<TcpStream>>) -> (){
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
    let mut content_str = String::from_utf8_lossy(&content);
    
    let mut stream_lock = stream.lock().await;

    stream_lock.write_all(&content).await;
    // stream_lock.write(b"test").await;

    println!("test:{}",content_str.as_ref());
}

pub async fn write_a_folder(stream: Arc<Mutex<TcpStream>>) -> (){
    let num_of_ports = 10;
    //creating ports
    let mut available_ports:Vec<u16> = vec![];
    for i in 0..num_of_ports{
        let sub_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = sub_listener.local_addr().unwrap().port();
        available_ports.push(port);
    }

    //writing list of ports to other device
    let mut main_stream_lock = stream.lock().await;
    let _ = main_stream_lock.write_all(b"FOLDER\n").await;
    for port in available_ports{
        let info = format!("PORT {}\n",port);
        let _ = main_stream_lock.write_all(info.as_bytes()).await;
    }
    let _ = main_stream_lock.write_all(b"END\n").await;


    if let Some(file_path) = FileDialog::new().set_directory("/".to_string()).pick_folder(){
        println!("folder: {}",file_path.display());
        if let Ok(mut files) = fs::read_dir(file_path).await{
            let mut i = 1;
            loop{
                let result = files.next_entry().await;
                match result{
                    Ok(file) => {
                        println!("File #{} : {:?}",i,file.unwrap().file_name());

                    }
                    Err(_) => println!("No more files")
                }
                i+=1;
            } 
        }
    } else{
        println!("No folder.")
    }
}


pub async fn write_file(stream: Arc<Mutex<TcpStream>>) -> (){
    let mut file_path = String::new();
    // let mut file = File::create_new("");
    let mut buffer = String::new();
    // }
    let file_path = FileDialog::new().set_directory("/".to_string()).pick_file();
    // let content = file.unwrap();
    let mut file_str = file_path.clone().unwrap();


    let file_name = file_str.file_name().unwrap().to_str().unwrap();
    let file_name_len = file_name.len() as u32; 

    let mut file_str = file_str.to_str().unwrap();
    println!("test:{}",file_str);

    let mut content: Vec<u8> = Vec::new();
    match fs::read(file_str).await{
        Ok(result) => {content = result;},
        Err(e) => eprintln!("ERROR:{}",e),
    }
    let mut content_str = String::from_utf8_lossy(&content);
    
    let mut stream_lock = stream.lock().await;

    stream_lock.write_all(&content).await;
    // stream_lock.write(b"test").await;

    println!("test:{}",content_str.as_ref());
}