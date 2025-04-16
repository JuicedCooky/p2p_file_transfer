use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};
use rfd::FileDialog;
use tokio::fs::{self, metadata};
use tokio::io::AsyncWriteExt;

use async_recursion::async_recursion;

pub async fn write_a_file_to_stream(stream: Arc<Mutex<TcpStream>>, path: Option<PathBuf>, print_file: bool) -> () {
    
    let file_path:PathBuf;

    // Set file path to write to host
    if path.is_none(){
        file_path = FileDialog::new().set_directory("/".to_string()).pick_file().unwrap();
    }
    else{
        file_path = path.unwrap();
    }
   
    //println!("{:?}", file_path);
    let cloned_file_path = file_path.clone(); 
    let file_str = cloned_file_path.file_name().unwrap().to_str().unwrap();

    let file_size = metadata(file_path).await.unwrap().len();

    let mut stream_lock = stream.lock().await;

    //println!("Acquired writer lock!");

    if print_file {
        println!("\nSending file {} to client\n", file_str);
    }
    
    // Send init message to host
    let _ = stream_lock.write_all(b"Init\n").await;
    let _ = stream_lock.flush().await;

    // Send FILENAME header to host 
    let filename_content = "FILENAME:".to_string() + file_str + "\n";
    let _ = stream_lock.write_all(filename_content.as_bytes()).await;
    
    // Send FILESIZE header to host
    let filesize_content = "FILESIZE:".to_string()  + &file_size.to_string() + "\n";
    let _ = stream_lock.write_all(filesize_content.as_bytes()).await;
    
    let _ = stream_lock.flush().await;

    // Read in file data as by to send to host
    let mut content: Vec<u8> = Vec::new();
    
    match fs::read(cloned_file_path).await{
        Ok(result) => {content = result;},
        Err(e) => eprintln!("ERROR:{}",e),
    }

    // Send file contents to host as byte data
    let _ = stream_lock.write_all(&content).await;
    let _ = stream_lock.flush().await;

    sleep(Duration::from_secs(1)).await;
}

#[async_recursion]
pub async fn write_a_folder_to_stream(stream: Arc<Mutex<TcpStream>>, folder_path: Option<PathBuf>, parent_folder: Option<String>) -> () {
    // Create ports
    let mut available_ports:Vec<u16> = vec![];
    let mut file_vector:Vec<PathBuf> = vec![];

    // Intialize variables to store information about selected folder
    let folder_name: &str;

    let clone_path: PathBuf;

    // Select and open folder
    let folder_selection: Option<PathBuf>;
    if folder_path.as_ref().is_none() {
        folder_selection = FileDialog::new().set_directory("/".to_string()).pick_folder();
    }
    else {
        folder_selection = Some(folder_path.clone().unwrap());
    }
    if let Some(file_path) = folder_selection{
        //println!("folder: {}",file_path.display());
        clone_path = file_path.clone();
        folder_name = clone_path.file_name().unwrap().to_str().unwrap();
        
        println!("\nSelected folder {}\n", folder_name);
        // Push the files in the directory into the file vectors
        if let Ok(mut files) = fs::read_dir(file_path.clone()).await{
            let mut _i = 1;
            loop{
                let result = files.next_entry().await;
                match result{
                    Ok(file) => {
                        if file.as_ref().is_some(){
                            if file.as_ref().unwrap().file_type().await.unwrap().is_file(){   
                                println!("Sending file {:?} to host", file.as_ref().unwrap().file_name());
                                file_vector.push(file.as_ref().unwrap().path());              
                            }
                            else if file.as_ref().unwrap().file_type().await.unwrap().is_dir(){
                                let mut parent: String = file_path.clone().file_name().unwrap().to_str().unwrap().to_string();
                                if parent_folder.is_some(){
                                    parent = parent_folder.clone().unwrap() + "\\" + parent.as_str();
                                }
                                write_a_folder_to_stream(
                                    stream.clone(), 
                                    Some(file_path.clone().join(file.as_ref().unwrap().path())), 
                                    Some(parent)
                                ).await;
                            }
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
            } 
        } else{
            println!("Folder not found");
        }

        println!("TEST:{},{}",folder_path.clone().unwrap_or_default().to_string_lossy(),parent_folder.clone().unwrap_or_default());

        // Get the number of files in the folder
        let file_count = file_vector.len();
        if file_count == 0 {
            println!("No files in folder to send.");
            let _  = stream.lock().await.write_all(b"END\n").await;
            let _ = stream.lock().await.flush().await;
            return;
        }

        //println!("File count is {}", file_count);
            
        // Initailize mutex for shared file
        let shared_file_vector = Arc::new(Mutex::new(file_vector));

        for _i in 0..file_count+1{
            let sub_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
            let port = sub_listener.local_addr().unwrap().port();
            available_ports.push(port);
 
            tokio::spawn({
                let shared_file_vector = Arc::clone(&shared_file_vector);
                async move{
                    if let Ok((sub_stream, _addr)) = sub_listener.accept().await {
                        //println!("conn on port {} from {}", port, addr);
                        let write_stream = Arc::new(Mutex::new(sub_stream));
                        loop {
                            let file_path = {
                                let mut shared_vector_lock = shared_file_vector.lock().await;
                                shared_vector_lock.pop()
                            };
                        
                            if let Some(path) = file_path {
                                //println!("writing...");
                                write_a_file_to_stream(write_stream.clone(), Some(path), false).await;
                                let _ = write_stream.lock().await.flush().await;
                            } else {
                                break;
                            }
                        }
                    }   
                }
            });      
        }
        
        // Send init message to host
        let _ = stream.lock().await.write_all(b"Init\n");
        let _ = stream.lock().await.flush();

        // Send Folder name header to host
        if parent_folder.is_some(){
            let folder_info = format!("FOLDER:{}\n",parent_folder.unwrap() + "\\" +folder_name);
            let _ = stream.lock().await.write_all(folder_info.as_bytes()).await;
        }
        else{
            let folder_info = format!("FOLDER:{}\n",folder_name);
            let _ = stream.lock().await.write_all(folder_info.as_bytes()).await;
        }
        
        // Writing list of ports to host
        for port in available_ports{
            let info = format!("PORT {}\n",port);
            let _ = stream.lock().await.write_all(info.as_bytes()).await;
            let _ = stream.lock().await.flush().await;
        }
        
        // Tell host to stop listening for ports
        if folder_path.as_ref().is_some(){
            let _  = stream.lock().await.write_all(b"END FOLDER\n").await;
            let _ = stream.lock().await.flush().await;
        }
        else{
            let _  = stream.lock().await.write_all(b"END\n").await;
            let _ = stream.lock().await.flush().await;
        }
        
    }

}
