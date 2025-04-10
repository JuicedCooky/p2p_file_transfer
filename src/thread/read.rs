// pub mod thread;

use tokio::net::TcpStream;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::Interest;



pub async fn read_from_stream(stream: Arc<Mutex<TcpStream>>) -> (){
    let stream_ip = stream.lock().await.peer_addr().unwrap().ip().to_string();
    println!("IP TEST:{}",stream_ip);
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
                            // println!("{TEST}");
                            if String::from_utf8_lossy(&buf).contains("FOLDER"){
                                println!("TEST");
                                loop{
                                    let n = read_lock.read(&mut buf).await;
                                    let string_buf = String::from_utf8_lossy(&buf);
                                    if(string_buf.contains("PORT") && n.is_ok()){
                                        let port = String::from(string_buf.to_string().strip_prefix("PORT").unwrap());
                                        //spawn a thread for each port connection
                                        let cloned_ip = stream_ip.clone();
                                        tokio::spawn(async move{
                                            parse_file_per_port(cloned_ip + ":" + port.clone().as_str().trim()).await;
                                        });
                                    }
                                }

                            }
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

pub async fn parse_file_per_port(address: String){
    println!("PARSING_PORT: {}",address);
    let stream = TcpStream::connect(address.trim()).await;
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