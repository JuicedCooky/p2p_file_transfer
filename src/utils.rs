// use std::io::Read;
use std::io::Write;
use tokio::fs::File;
use tokio::net::TcpStream;
use std::error::Error;
use std::path::Path;


pub async fn display_options(stream: &TcpStream) -> (){
    let mut choice = String::new();
    // let mut file = File::create_new("");
    while choice.is_empty()
    {
        println!("What would you like to do?");
        println!("1. Select File to send to host");
        print!("Enter choice:");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice);
    }
    match choice.as_str(){
        "1" => file(stream).await,
        _ => println!("Error: INVALID CHOICE."),
    }
}
 
pub async fn file(stream: &TcpStream) -> (){
    let mut file_path = String::new();
    let mut file = File::create_new("");
    print!("Enter path to file:");
    std::io::stdout().flush();

    std::io::stdin().read_line(&mut file_path);
    
    if Path::new(&file_path).exists()
    {
        file = File::create_new(&file_path);
    }
}