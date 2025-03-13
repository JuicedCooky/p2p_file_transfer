use get_if_addrs::Interface;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;

use tokio::io::AsyncReadExt;

use std::error::Error;

// use std::fs;

// use rfd::FileDialog;

use get_if_addrs::get_if_addrs;
use local_ip_address::local_ip;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {

    println!("TESTING SENDER");

    //run ncat -l 6142 (in wsl or linux)
    //127.0.0.1 is the ip address of the current device

    //0.0.0.0 sets listener on all network addresses
    let listener = TcpListener::bind("0.0.0.0:6142").await?;
    // let interfaces = get_if_addrs().unwrap();
    // for iface in interfaces{
    //     println!("Interface: {}, address: {}",iface.name,iface.addr.ip());
    //     if(iface.name == "eth0"){
    //         println!("Server running\n IP-ADDR: {}", iface.addr.ip());
    //     }
    // }
    match local_ip(){
        Ok(ip) => println!("Server running\nLocal Address: {}", ip),
        Err(e) => println!("Could not start server!\n{}",e.to_string()),
    }

    loop {
        match listener.accept().await {
            Ok((stream,addr)) => 
            {
                println!("Connection from {}", addr);
            }
            Err(e) => eprintln!("Failed connection :{}",e),
        }
    }
    Ok(())
}