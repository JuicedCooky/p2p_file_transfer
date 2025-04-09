use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use rfd::FileDialog;
use tokio::fs;
use tokio::io::AsyncWriteExt;


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
    if let Some(file_path) = FileDialog::new().set_directory("/".to_string()).pick_folder(){
        println!("folder: {}",file_path.display());
    } else{
        println!("No folder.")
    }
}