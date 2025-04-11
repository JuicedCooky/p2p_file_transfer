// pub mod thread;

use bytes::Bytes;
use rfd::FileDialog;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::task;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::usize;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::Interest;

use crate::thread::read;


pub async fn read_line_from_stream(
    stream: Arc<Mutex<TcpStream>>,
    line_buf: &mut String
) -> tokio::io::Result<usize> {
    let mut buffer = [0u8; 1024];
    let mut total_read = 0;

    loop {
        let n = stream.lock().await.read(&mut buffer).await?;

        if n == 0 {
            break; // connection closed
        }

        total_read += n;

        let chunk = String::from_utf8_lossy(&buffer[..n]);
        if let Some(pos) = chunk.find('\n') {
            line_buf.push_str(&chunk[..=pos]);
            break;
        } else {
            line_buf.push_str(&chunk);
        }
    }

    Ok(total_read)
}


pub async fn read_from_stream(stream: Arc<Mutex<TcpStream>>, folder_path:Option<String>) -> (){

    let folder_path = folder_path.clone();
    let stream_ip = stream.lock().await.peer_addr().unwrap().ip().to_string();
    // let mut read_lock = stream.lock().await;

    let ip_add = stream.lock().await.local_addr().unwrap();


    let mut line = String::new();
    let mut folder_name:String = ("").to_string();
    println!("IP TEST:{}",stream_ip);

    let mut save_location = PathBuf::from("./");
    if folder_path.is_none(){
        save_location = task::spawn_blocking(move ||{
            return FileDialog::new().set_title("Choose save location").set_directory("/".to_string()).pick_folder().unwrap();
        }).await.unwrap();
    }
    
    loop{
        line.clear();
        match read_line_from_stream(stream.clone(), &mut line).await{
            Ok(0) =>{
                println!("CLOSED");
                break;
            }
            Ok(_) => {
                println!("Line:{}",line);
                //for reading multiple files
                if line.contains("FOLDER"){
                    let folder_name = line.strip_prefix("FOLDER:").unwrap().to_string();
                    let save_location =  save_location.to_str().unwrap().to_string() + "\\" + folder_name.as_str(); 
                    println!("TEST MAKING LOCATION:{}",save_location);
                    fs::create_dir(save_location.clone().trim()).await;
                    println!("TEST");
                    loop{
                        line.clear();
                        read_line_from_stream(stream.clone(), &mut line).await;
                        if !line.is_empty(){
                            println!("Line:{}",line);
                        }
                        
                        if line.contains("PORT"){
                            let port = line.strip_prefix("PORT ").unwrap().to_string();
                            let ip = ip_add.ip().to_string();
                            let concat_port = ip + ":" + &port; 
                            let parsed_port = concat_port.clone();

                            println!("Parsed Port:{}",parsed_port.trim());
                            let cloned_folder_name = folder_name.clone();
                            let cloned_save_location = Some(save_location.clone());

                            tokio::spawn({
                                async move{
                                    parse_file_per_port(parsed_port,cloned_save_location).await;
                                }
                            });
                        }
                    }
                }
                //reading a single file
                else if line.contains("FILE"){
                    line.clear();
                    read_line_from_stream(stream.clone(), &mut line).await;
                    let cloned_line = line.clone();
                    let file_name = cloned_line.strip_prefix("FILENAME:").unwrap().trim();
                    println!("FILENAME:{}", file_name);

                    line.clear();
                    read_line_from_stream(stream.clone(), &mut line).await;
                    let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
                    println!("FILESIZE:{}", file_size);


                    let mut received: usize = 0;
                    let file_size_usize = file_size.parse::<usize>().unwrap(); 
                    let mut file : File;
                    if folder_path.is_none(){
                        file = File::create("./".to_string() + file_name).await.unwrap();
                    }
                    else{
                        file = File::create(folder_path.as_ref().unwrap().to_string() + "/" + file_name).await.unwrap();
                    }

                    let mut buffer = [0u8; 4096];
                    while received < file_size_usize{
                        let n = stream.lock().await.read(&mut buffer).await.unwrap();

                        if n == 0{
                            break;
                        }

                        received += n;

                        file.write_all(&mut buffer[..n]).await;
                    }
                }
            }
            Err(e) => {
                println!("ERROR:{}",e);
            }
        }
    }
}

pub async fn read_file_from_stream_no_async(mut stream: TcpStream,folder_path:Option<String>){
    let folder_path = folder_path.clone();
    // let mut read_lock = stream.lock().await;
    


    let mut line = String::new();
    let mut folder_name:String = ("").to_string();

    // let mut save_location = PathBuf::from("./");
    // if folder_path.is_none(){
    //     save_location = task::spawn_blocking(move ||{
    //         return FileDialog::new().set_title("Choose save location").set_directory("/".to_string()).pick_folder().unwrap();
    //     }).await.unwrap();
    // }
    
    loop{
        line.clear();
        let mut buf_reader = BufReader::new(&mut stream);
        match buf_reader.read_line(&mut line).await{
            Ok(0) =>{
                println!("CLOSED");
                break;
            }
            Ok(_) => {
                // println!("Line:{}",line);s
                //for reading multiple files
                //reading a single file
                if line.contains("FILE"){
                    line.clear();
                    buf_reader.read_line(&mut line).await;
                    let cloned_line = line.clone();
                    let file_name = cloned_line.strip_prefix("FILENAME:").unwrap().trim();
                    println!("FILENAME:{}", file_name);

                    line.clear();
                    buf_reader.read_line(&mut line).await;
                    let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
                    println!("FILESIZE:{}", file_size);


                    let mut received: usize = 0;
                    let file_size_usize = file_size.parse::<usize>().unwrap(); 
                    let mut file : File;
                    if folder_path.is_none(){
                        file = File::create("./".to_string() + file_name).await.unwrap();
                    }
                    else{
                        let file_path = PathBuf::from(folder_path.as_ref().unwrap().to_string().trim()).join(file_name.trim());
                        println!("FOLDER PATH:{}",file_path.to_str().unwrap());
                        file = File::create(file_path).await.unwrap();
                    }
                    drop(buf_reader);
                    let mut buffer = [0u8; 4096];
                    while received < file_size_usize{
                        let n = stream.read(&mut buffer).await.unwrap();

                        if n == 0{
                            break;
                        }

                        received += n;

                        file.write_all(&mut buffer[..n]).await;
                    }
                }
            }
            Err(e) => {
                println!("ERROR:{}",e);
            }
        }
    }
} 

pub async fn parse_file_per_port(address: String, folder_path:Option<String>){
    println!("PARSING_PORT:{}",address.trim());
    // print
    match TcpStream::connect(address.trim()).await{
        Ok(stream)=>{
            // let stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(stream));
            
            println!("connected to port");
            read_file_from_stream_no_async(stream, folder_path).await;
        }
        Err(e) => {println!("Failed to connect to port:{}",e);}
    }
}