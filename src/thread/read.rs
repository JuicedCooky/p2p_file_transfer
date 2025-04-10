// pub mod thread;

use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::Interest;

use crate::thread::read;



pub async fn read_from_stream(stream: Arc<Mutex<TcpStream>>) -> (){
    let stream_ip = stream.lock().await.peer_addr().unwrap().ip().to_string();
    let mut read_lock = stream.lock().await;

    let ip_add = read_lock.local_addr().unwrap();

    let mut reader = BufReader::new(&mut *read_lock);

    let mut line = String::new();
    println!("IP TEST:{}",stream_ip);
    loop{
        line.clear();
        match reader.read_line(&mut line).await{
            Ok(0) =>{
                println!("CLOSED");
                break;
            }
            Ok(_) => {
                println!("Line:{}",line);
                if line.contains("PORT"){
                    let port = line.strip_prefix("PORT ").unwrap().to_string();
                    let ip = ip_add.ip().to_string();
                    let concat_port = ip + ":" + &port; 
                    let parsed_port = concat_port;
                    
                    println!("Parsed Port:{}",parsed_port.trim());
                    tokio::spawn(async {
                        parse_file_per_port(parsed_port).await;

                    });
                }
            }
            Err(e) => {
                println!("ERROR:{}",e);
            }
        }

        // match read_lock.ready(Interest::READABLE).await{
        //     Ok(_) => {
        //         let mut buf =[0;4096];
        //         // if(read_lock.poll_read_ready(cx))
                
        //         match read_lock.read(&mut buf).await{
        //             Ok(n) => {
        //                 if(n>0)
        //                 {
        //                     println!("Read from {}:{}",read_lock.local_addr().unwrap(),String::from_utf8_lossy(&buf));
        //                     // println!("{TEST}");
        //                     if String::from_utf8_lossy(&buf).contains("FOLDER"){
        //                         println!("TEST");
        //                         loop{   
        //                             let n = read_lock.read(&mut buf).await;
        //                             let string_buf = String::from_utf8_lossy(&buf[..*n.as_ref().unwrap()]);
        //                             if(string_buf.contains("PORT") && n.is_ok()){
        //                                 let port = String::from(string_buf[..n.unwrap()].strip_prefix("PORT").unwrap());
        //                                 //spawn a thread for each port connection
        //                                 let cloned_ip = stream_ip.clone();
        //                                 tokio::spawn(async move{
        //                                     parse_file_per_port(cloned_ip + ":" + port.clone().as_str().trim()).await;
        //                                 });
        //                             }
        //                             else if string_buf.contains("END") {
        //                                 break;
        //                             }
        //                         }

        //                     }
        //                 }
        //             }
        //             // Ok(_) => {break;}
        //             Err(e) => {eprintln!("Error: {}",e);}
        //         }
        //     }
        //     Err(_) => {}
        // }
    }
}

pub async fn parse_file_per_port(address: String){
    println!("PARSING_PORT:{}",address.trim());
    let stream = TcpStream::connect(address.trim()).await;
    // print
    match stream{
        Ok(mut stream)=>{println!("connected to port");}
        Err(e) => {println!("Failed to connect to port:{}",e);}
    }
}

pub async fn read_to_file(stream: Arc<Mutex<TcpStream>>) -> (){
    loop{
        let mut read_lock = stream.lock().await;
        match read_lock.ready(Interest::READABLE).await{
            Ok(_) => {
                let mut buf =[0;4096];
                // if(read_lock.poll_read_ready(cx))
                
                match read_lock.read(&mut buf).await{
                    Ok(n) => {
                        
                    }
                    // Ok(_) => {break;}
                    Err(e) => {eprintln!("Error: {}",e);}
                }
            }
            Err(_) => {}
        }
    }
}

pub async fn read_from_stream_folder(stream: Arc<Mutex<TcpStream>>) -> (){
    loop{
        let mut read_lock = stream.lock().await;
        match read_lock.ready(Interest::READABLE).await{
            Ok(_) => {
                let mut buf =[0;4096];
                // if(read_lock.poll_read_ready(cx))
                
                match read_lock.read(&mut buf).await{
                    Ok(n) => {
                        if(n>0)
                        {
                            println!("Read from {}:{}",read_lock.local_addr().unwrap(),String::from_utf8_lossy(&buf));
                        }
                    }
                    // Ok(_) => {break;}
                    Err(e) => {eprintln!("Error: {}",e);}
                }
            }
            Err(_) => {}
        }
    }
}