// pub mod thread;

use bytes::buf;
use bytes::Buf;
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
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::usize;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::Interest;

use crate::thread::read;

pub async fn read_file_from_stream(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf) -> () {

    // Print IP address as test
    let stream_ip = stream.lock().await.peer_addr().unwrap().ip().to_string();
    //println!("IP TEST:{}",stream_ip);

    //println!("Saving file to location {:?}", file_save_location);

    // Initialize stream lock and buffer reader to read data from client
    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);

    //println!("Acquired reader lock");

    let mut line = String::new();

    match reader.read_line(&mut line).await { 
        Ok(0) =>{
            println!("CONNECTION CLOSED ABRUPTLY");
            return;
        },
        Ok(_) => { 

            //println!("Init message is {}", line);

            // Clear line buffer
            line.clear(); 
           
            // Get FILENAME header
            reader.read_line(&mut line).await;
            let header_line = line.clone();
            //println!("Received raw line {}", header_line);
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            //println!("FILENAME:{}", file_name);
            
            // Clear line buffer
            line.clear(); 

            // Get FILESIZE header
            reader.read_line(&mut line).await;
            //println!("Received raw line {}", line);
            let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
            //println!("FILESIZE:{}", file_size);

            // Prepare to receive file data as byte data
            let mut received: usize = 0;
            let file_size_usize = file_size.parse::<usize>().unwrap();
            
            let file_path = file_save_location.join(file_name);
            //println!("Writing to file path: {}", file_path.to_string_lossy());
            let mut file = File::create(file_path).await.unwrap();

            line.clear();
            let mut buffer = [0u8; 4096];
            //println!("Starting file read loop (expecting {} bytes)...", file_size_usize);

            while received < file_size_usize{
                let mut max_size = std::cmp::min(file_size_usize-received,buffer.len());
                let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                //println!("Read {} bytes: {:?}", n, &buffer[..n]);

                if n == 0{
                    break;
                }

                received += n;

                //println!("Received {}/{} bytes", received, file_size_usize);

                file.write_all(&mut buffer[..n]).await;
            }

            println!("\nReceived file named {} of size {} bytes\n", file_name, file_size_usize);

        },
        Err(e) => {
            println!("ERROR:{}",e);
            return;
        }

    }

}

pub async fn read_folder_from_stream(stream: Arc<Mutex<TcpStream>>, outgoing_adder:String, folder_save_location: PathBuf) -> () {

    // Initialize stream lock and buffer reader to read data from client
    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);

    let mut line = String::new();

    match reader.read_line(&mut line).await {
        Ok(0) =>{
            // println!("CONNECTION CLOSED ABRUPTLY");
            return;
        },
        Ok(_) => {
            //println!("Current contents of line are {}", line);

            // line.clear();

            // Get Folder name header
            let folder_name = line.strip_prefix("FOLDER:").unwrap().to_string();
            let save_location =  folder_save_location.to_str().unwrap().to_string() + "\\" + folder_name.as_str();
            println!("\nReceiving folder {}", folder_name); 
            //println!("Received Folder Location:{}",save_location);
            fs::create_dir(save_location.clone().trim()).await;

            // Parse list of available ports sent by the client
            loop {

                line.clear();
                reader.read_line(&mut line).await;

                if line.contains("END"){
                    //println!("All ports assigned");
                    break;
                }

                if line.contains("PORT"){ 
                    // Parse value of port
                    //println!("Received {}", line);
                    let port = line.strip_prefix("PORT ").unwrap().to_string();
                    let concat_port = outgoing_adder.to_owned() + ":" + &port; 
                    let parsed_port = concat_port.clone();
                    //println!("Parsed Port:{}",parsed_port.trim());

                    let cloned_save_location = save_location.clone().trim().to_string();
                    tokio::spawn({
                        async move{
                            parse_file_per_port_stream(parsed_port,cloned_save_location).await;
                        }
                    });
                }

            }
        },
        Err(e) => {
            println!("ERROR:{}",e);
            return;
        }
    }

}

pub async fn read_file_from_stream_direct(mut stream: TcpStream, file_save_location: PathBuf) {
    //println!("Connected: local = {}, peer = {}", stream.local_addr().unwrap(), stream.peer_addr().unwrap());
    //println!("Saving file to location {:?}", file_save_location);

    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();

    match reader.read_line(&mut line).await {
        Ok(0) => {
            //println!("CONNECTION CLOSED ABRUPTLY");
            return;
        }
        Ok(_) => {
            //println!("Init message is {}", line);
            line.clear();

            reader.read_line(&mut line).await;
            let header_line = line.clone();
            //println!("Received raw line {}", header_line);
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            line.clear();

            reader.read_line(&mut line).await;
            //println!("Received raw line {}", line);
            let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
            let file_size_usize = file_size.parse::<usize>().unwrap();

            let file_path = file_save_location.join(file_name);
            let mut file = File::create(file_path).await.unwrap();

            let mut buffer = [0u8; 4096];
            let mut received = 0;
            //println!("Starting file read loop (expecting {} bytes)...", file_size_usize);

            while received < file_size_usize {
                let max_size = std::cmp::min(file_size_usize - received, buffer.len());
                let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                if n == 0 {
                    break;
                }

                received += n;
                //println!("Received {}/{} bytes", received, file_size_usize);

                file.write_all(&buffer[..n]).await.unwrap();
            }

            println!("Received file named: {} of size {} bytes", file_name, file_size_usize);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
}


pub async fn parse_file_per_port_stream(address: String, folder_path: String) {
    //println!("PARSING_PORT:{}",address.trim());
    match TcpStream::connect(address.trim()).await {
        Ok(stream) => {
            //println!("Connected to port {:?}", stream);
            read_file_from_stream_direct(stream, PathBuf::from(folder_path)).await;
        },
        Err(e) => {println!("Failed to connect to port:{}",e);}
    }
}

// Legacy
pub async fn read_from_stream(stream: Arc<Mutex<TcpStream>>, outgoing_adder:String, folder_path:Option<String>) -> (){

    let folder_path = folder_path.clone();
    let stream_ip = stream.lock().await.peer_addr().unwrap().ip().to_string();

    let mut line = String::new();
    println!("IP TEST:{}",stream_ip);

    let mut save_location = PathBuf::from("./");
    if folder_path.is_none(){
        save_location = task::spawn_blocking(move ||{
            return FileDialog::new().set_title("Choose save location").set_directory("/".to_string()).pick_folder().unwrap();
        }).await.unwrap();
    }

    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);
    
    loop{
        line.clear();
        match reader.read_line(&mut line).await {
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
                    println!("Received Folder Location:{}",save_location);
                    fs::create_dir(save_location.clone().trim()).await;
                
                    line.clear();
                    loop{
                        println!("test:loop");
                        line.clear();
                        reader.read_line(&mut line).await;

                        if line.eq("END"){
                            println!("No more ports");
                            break;
                        }

                        if !line.is_empty(){
                            println!("Line:{}",line);
                        }
                        
                        if line.contains("PORT"){
                            let port = line.strip_prefix("PORT ").unwrap().to_string();
                            let concat_port = outgoing_adder.to_owned() + ":" + &port; 
                            let parsed_port = concat_port.clone();

                            println!("Parsed Port:{}",parsed_port.trim());
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

                    // Get FILENAME header
                    reader.read_line(&mut line).await;
                    let header_line = line.clone();
                    let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
                    println!("FILENAME:{}", file_name);
                    line.clear();

                    // Get FILESIZE header
                    reader.read_line(&mut line).await;
                    let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
                    println!("FILESIZE:{}", file_size);
                    
                    // Prepare to receive file data as byte data
                    let mut received: usize = 0;
                    let file_size_usize = file_size.parse::<usize>().unwrap();
                    
                    let file_path = save_location.join(file_name);
                    println!("Writing to file path: {}", file_path.to_string_lossy());
                    let mut file = File::create(file_path).await.unwrap();
        
                    line.clear();
                    let mut buffer = [0u8; 4096];
                    println!("Starting file read loop (expecting {} bytes)...", file_size_usize);
                    while received < file_size_usize{
                        let mut max_size = std::cmp::min(file_size_usize-received,buffer.len());
                        let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                        //println!("Read {} bytes: {:?}", n, &buffer[..n]);

                        if n == 0{
                            break;
                        }

                        received += n;

                        println!("Received {}/{} bytes", received, file_size_usize);

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

// Legacy
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
    let mut buf_reader = BufReader::new(&mut stream);
    
    loop{
        line.clear();
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
                    let mut buffer = [0u8; 4096];
                    while received < file_size_usize{
                        let mut max_size = std::cmp::min(file_size_usize-received,buffer.len());
                        let n = buf_reader.read(&mut buffer[..max_size]).await.unwrap();

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

// Legacy
pub async fn parse_file_per_port(address: String, folder_path:Option<String>) {
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

// Legacy
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