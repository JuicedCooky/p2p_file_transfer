// pub mod thread;

use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::Interest;


pub async fn read_from_stream(stream: Arc<Mutex<TcpStream>>) -> (){
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