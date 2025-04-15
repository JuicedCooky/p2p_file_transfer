use core::num;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use local_ip_address::local_ip;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};
use rfd::FileDialog;
use tokio::fs::{self, metadata};
use tokio::io::AsyncWriteExt;

use crate::main;

pub async fn write_a_file_to_stream(stream: Arc<Mutex<TcpStream>>) -> () {
    
    // Select file to write to host
    let file_path = FileDialog::new().set_directory("/".to_string()).pick_file().unwrap();

    println!("{:?}", file_path);
    let cloned_file_path = file_path.clone(); 
    let file_str = cloned_file_path.file_name().unwrap().to_str().unwrap();

    let file_size = metadata(file_path).await.unwrap().len();

    let mut stream_lock = stream.lock().await;

    println!("Acquired writer lock!");

    // Send init message to host
    stream_lock.write_all(b"Init\n").await;
    stream_lock.flush().await;

    // Send FILENAME header to host 
    let filename_content = "FILENAME:".to_string() + file_str + "\n";
    stream_lock.write_all(filename_content.as_bytes()).await;
    println!("Sent header {}", filename_content);

    // Send FILESIZE header to host
    let filesize_content = "FILESIZE:".to_string()  + &file_size.to_string() + "\n";
    stream_lock.write_all(filesize_content.as_bytes()).await;
    println!("Sent header {}", filesize_content);

    stream_lock.flush().await;

    // Read in file data as by to send to host
    let mut content: Vec<u8> = Vec::new();
    
    match fs::read(cloned_file_path).await{
        Ok(result) => {content = result;},
        Err(e) => eprintln!("ERROR:{}",e),
    }

    // Send file contents to host as byte data
    stream_lock.write_all(&content).await;
    stream_lock.flush().await;

    sleep(Duration::from_secs(1)).await;
}

pub async fn write_a_folder_to_stream(stream: Arc<Mutex<TcpStream>>) -> () {
    
}

pub async fn write_a_file(stream: Arc<Mutex<TcpStream>>, path: Option<PathBuf>) -> (){

    let file_path:PathBuf;
    
    if path.is_none(){
        file_path = FileDialog::new().set_directory("/".to_string()).pick_file().unwrap();
    }
    else{
        file_path = path.unwrap();
    }
    println!("{:?}", file_path);
    // let content = file.unwrap();
    let cloned_file_path = file_path.clone(); 
    let file_str = cloned_file_path.file_name().unwrap().to_str().unwrap();

    let file_size = metadata(file_path).await.unwrap().len();
    //println!("FILE_NAME:{}\nFILE_SIZE:{}",file_str,file_size);

    let mut stream_lock = stream.lock().await;
    
    // Send type header to host
    stream_lock.write_all(b"FILE\n").await;
    stream_lock.flush().await;
   
    // Send FILENAME header to host 
    let filename_content = "FILENAME:".to_string() + file_str + "\n";
    stream_lock.write_all(filename_content.as_bytes()).await;
    println!("Sent header {}", filename_content);
    
    // Send FILESIZE header to host
    let filesize_content = "FILESIZE:".to_string()  + &file_size.to_string() + "\n";
    stream_lock.write_all(filesize_content.as_bytes()).await;
    println!("Sent header {}", filesize_content);
    
    // Send new line characters to clear host buffer
    stream_lock.write_all(b"\n\n");
    

    stream_lock.flush().await;

    // Read in file data as by to send to host
    let mut content: Vec<u8> = Vec::new();
    
    match fs::read(cloned_file_path).await{
        Ok(result) => {content = result;},
        Err(e) => eprintln!("ERROR:{}",e),
    }
    //let mut content_str = String::from_utf8_lossy(&content);
    
    // Send file contents to host as byte data
    stream_lock.write_all(&content).await;
    stream_lock.flush().await;
    //println!("Finished sending file content, sleeping...");
    sleep(Duration::from_secs(1)).await;
 
    //println!("Sent following file contents:");
    //println!("{}",content_str.as_ref());
}

pub async fn write_a_folder(stream: Arc<Mutex<TcpStream>>) -> (){
    let num_of_ports = 10;
    //creating ports
    let mut available_ports:Vec<u16> = vec![];
    let mut file_vector:Vec<PathBuf> = vec![];
    let mut folder_name: &str = "";
    let clone_path: PathBuf;

    if let Some(file_path) = FileDialog::new().set_directory("/".to_string()).pick_folder(){
        println!("folder: {}",file_path.display());
        clone_path = file_path.clone();
        folder_name = clone_path.file_name().unwrap().to_str().unwrap();
        if let Ok(mut files) = fs::read_dir(file_path).await{
            let mut i = 1;
            loop{
                println!("START OF LOOP");
                let result = files.next_entry().await;
                match result{
                    Ok(file) => {
                        if file.as_ref().is_some(){
                            println!("File #{} : {:?}",i,file.as_ref().unwrap().file_name());
                            file_vector.push(file.as_ref().unwrap().path());                        
                        }
                        else{
                            break;
                        }
                    }
                    Err(_) => {
                        println!("No more files");
                        break;
                    }
                }
                i+=1;
            } 
        }
    } else{
        println!("No folder.")
    }

    //mutex for shared file
    let shared_file_vector = Arc::new(Mutex::new(file_vector));
    
    for i in 0..num_of_ports{
        let sub_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = sub_listener.local_addr().unwrap().port();
        available_ports.push(port);


        tokio::spawn({
            let shared_file_vector = Arc::clone(&shared_file_vector);
            async move{
            if let Ok((sub_stream, addr)) = sub_listener.accept().await {
                println!("conn on port {} from {}", port, addr);
                let write_stream = Arc::new(Mutex::new(sub_stream));
                loop{
                    let mut shared_vector_lock = shared_file_vector.lock().await;
                    if !shared_vector_lock.is_empty(){
                        let file_path = shared_vector_lock.pop();
                        if file_path.is_some(){
                            println!("writing...");
                            write_a_file(write_stream.clone(), file_path).await;
                            write_stream.lock().await.flush().await;
                        }
                        else{
                            break;
                        }
                    }
                    drop(shared_vector_lock);
                }
            }   
        }});
    }

    //writing list of ports to other device
    // let mut main_stream_lock = stream.lock().await;
    let folder_info = format!("FOLDER:{}\n",folder_name);
    let _ = stream.lock().await.write_all(folder_info.as_bytes()).await;
    stream.lock().await.flush().await;
    let ip = local_ip().unwrap();
    println!("LOCAL IP:{}",ip);
    for port in available_ports{
        let info = format!("PORT {}\n",port);
        let _ = stream.lock().await.write_all(info.as_bytes()).await;
        stream.lock().await.flush().await;
        println!("TESTING PORT:{}",port);
    }
    let _ = stream.lock().await.write_all(b"END\n").await;
    stream.lock().await.flush().await;
    println!("TEST END");

}

