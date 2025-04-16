use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use std::path::PathBuf;
use std::sync::Arc;
use std::usize;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;

use crate::dual;

pub async fn read_file_from_stream(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf) -> () {

    // Initialize stream lock and buffer reader to read data from client
    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);

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
            let _ = reader.read_line(&mut line).await;
            let header_line = line.clone();
            //println!("Received raw line {}", header_line);
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            //println!("FILENAME:{}", file_name);
            
            // Clear line buffer
            line.clear(); 

            // Get FILESIZE header
            let _ = reader.read_line(&mut line).await;
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
                let max_size = std::cmp::min(file_size_usize-received,buffer.len());
                let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                if n == 0{
                    break;
                }

                received += n;

                let _ = file.write_all(&mut buffer[..n]).await;
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
    loop{
        match reader.read_line(&mut line).await {
            Ok(0) =>{
                return;
            },
            Ok(_) => {
                if line.trim().eq("END"){
                    return;
                }
                println!("Current contents of line are {}", line);

                // line.clear();

                // Get Folder name header
                let folder_name = line.strip_prefix("FOLDER:").unwrap().to_string();
                let save_location =  folder_save_location.to_str().unwrap().to_string() + "\\" + folder_name.as_str();
                println!("\nReceiving folder {}", folder_name);
                //println!("Received Folder Location:{}",save_location);
                match fs::create_dir_all(save_location.clone().trim()).await {
                    Ok(_) => {},
                    Err(e) => {println!("Error creating directory:{}",e)}
                }

                // Parse list of available ports sent by the client
                loop {

                    line.clear();
                    let _ = reader.read_line(&mut line).await;

                    if line.contains("END FOLDER"){
                        line.clear();
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

}

pub async fn read_file_from_stream_direct(mut stream: TcpStream, file_save_location: PathBuf) {
    //println!("Connected: local = {}, peer = {}", stream.local_addr().unwrap(), stream.peer_addr().unwrap());
    //println!("Saving file to location {:?}", file_save_location);

    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();

    match reader.read_line(&mut line).await {
        Ok(0) => {
            return;
        }
        Ok(_) => {
            line.clear();

            let _ = reader.read_line(&mut line).await;
            let header_line = line.clone();
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            line.clear();

            let _ = reader.read_line(&mut line).await;
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

pub async fn read_file_from_stream_dual(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf, log_path: PathBuf) -> () {

    // Initialize stream lock and buffer reader to read data from client
    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);

    let mut line = String::new();

    match reader.read_line(&mut line).await { 
        Ok(0) =>{
            dual::log_to_file(&log_path, &format!("CONNECTION CLOSED ABRUPTLY"));
            return;
        },
        Ok(_) => { 
            // Clear line buffer
            line.clear(); 
           
            // Get FILENAME header
            let _ = reader.read_line(&mut line).await;
            let header_line = line.clone();
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            
            // Clear line buffer
            line.clear(); 

            // Get FILESIZE header
            let _ = reader.read_line(&mut line).await;
            let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
         
            // Prepare to receive file data as byte data
            let mut received: usize = 0;
            let file_size_usize = file_size.parse::<usize>().unwrap();
            
            let file_path = file_save_location.join(file_name);
            let mut file = File::create(file_path).await.unwrap();

            line.clear();
            let mut buffer = [0u8; 4096];
       
            while received < file_size_usize{
                let max_size = std::cmp::min(file_size_usize-received,buffer.len());
                let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                if n == 0{
                    break;
                }

                received += n;

                let _ = file.write_all(&mut buffer[..n]).await;
            }

            dual::log_to_file(&log_path, &format!("\nReceived file named {} of size {} bytes\n", file_name, file_size_usize));

        },
        Err(e) => {
            dual::log_to_file(&log_path, &format!("ERROR:{}",e));
            return;
        }

    }

}

pub async fn read_folder_from_stream_dual(stream: Arc<Mutex<TcpStream>>, outgoing_adder:String, folder_save_location: PathBuf, 
    log_path: PathBuf) -> () {

    // Initialize stream lock and buffer reader to read data from client
    let mut stream_lock = stream.lock().await;
    let mut reader = BufReader::new(&mut *stream_lock);

    let mut line = String::new();
    loop{
        match reader.read_line(&mut line).await {
            Ok(0) =>{
                return;
            },
            Ok(_) => {
                if line.trim().eq("END"){
                    return;
                }
                // Get Folder name header
                let folder_name = line.strip_prefix("FOLDER:").unwrap().to_string();
                let save_location =  folder_save_location.to_str().unwrap().to_string() + "\\" + folder_name.as_str();
                dual::log_to_file(&log_path, &format!("\nReceiving folder {}", folder_name));

                match fs::create_dir_all(save_location.clone().trim()).await {
                    Ok(_) => {},
                    Err(e) => {dual::log_to_file(&log_path, &format!("Error creating directory:{}",e));}
                }

                // Parse list of available ports sent by the client
                loop {
                    line.clear();

                    let _ = reader.read_line(&mut line).await;

                    if line.contains("END FOLDER"){
                        line.clear();
                        break;
                    }

                    if line.contains("END"){
                        return;
                    }

                    if line.contains("PORT"){ 
                        let port = line.strip_prefix("PORT ").unwrap().to_string();
                        let concat_port = outgoing_adder.to_owned() + ":" + &port; 
                        let parsed_port = concat_port.clone();
                       
                        let cloned_save_location = save_location.clone().trim().to_string();
                        let log_path_clone = log_path.clone();

                        tokio::spawn({
                            async move{
                                parse_file_per_port_stream_dual(parsed_port,cloned_save_location, log_path_clone).await;
                            }
                        });
                    }

                }
            },
            Err(e) => {
                dual::log_to_file(&log_path, &format!("ERROR:{}",e));

                return;
            }
        }
    }

}

pub async fn parse_file_per_port_stream_dual(address: String, folder_path: String, log_path: PathBuf) {
    match TcpStream::connect(address.trim()).await {
        Ok(stream) => {
            read_file_from_stream_direct_dual(stream, PathBuf::from(folder_path), log_path).await;
        },
        Err(e) => {println!("Failed to connect to port:{}",e);}
    }
}

pub async fn read_file_from_stream_direct_dual(mut stream: TcpStream, file_save_location: PathBuf, log_path: PathBuf) {
    
    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();

    match reader.read_line(&mut line).await {
        Ok(0) => {
            return;
        }
        Ok(_) => {
            line.clear();

            let _ = reader.read_line(&mut line).await;
            let header_line = line.clone();
            let file_name = header_line.strip_prefix("FILENAME:").unwrap().trim();
            line.clear();

            let _ = reader.read_line(&mut line).await;
            let file_size = line.strip_prefix("FILESIZE:").unwrap().trim();
            let file_size_usize = file_size.parse::<usize>().unwrap();

            let file_path = file_save_location.join(file_name);
            let mut file = File::create(file_path).await.unwrap();

            let mut buffer = [0u8; 4096];
            let mut received = 0;
            
            while received < file_size_usize {
                let max_size = std::cmp::min(file_size_usize - received, buffer.len());
                let n = reader.read(&mut buffer[..max_size]).await.unwrap();

                if n == 0 {
                    break;
                }

                received += n;
             
                file.write_all(&buffer[..n]).await.unwrap();
            }

            dual::log_to_file(&log_path, &format!("Received file named: {} of size {} bytes", file_name, file_size_usize));
        }
        Err(e) => {
            dual::log_to_file(&log_path, &format!("ERROR: {}", e));
        }
    }
}

