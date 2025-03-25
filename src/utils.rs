// use std::io::Read;
use std::io::Write;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use std::error::Error;
use std::path::Path;

use rfd::FileDialog;


pub async fn display_options(stream: &TcpStream) -> (){
    let mut choice = String::new();
    // let mut file = File::create_new("");
    loop {
        println!("What would you like to do?");
        println!("1. Select File to send to host");
        print!("Enter choice:");
        
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice);
        choice = choice.trim().to_string();
        
        match choice.as_str(){
            "1" => file(stream).await,
            _ => println!("Error: INVALID CHOICE."),
        }
    }
}
 
pub async fn file(stream: &TcpStream) -> (){
    let mut file_path = String::new();
    // let mut file = File::create_new("");
    let mut buffer = String::new();
    
    // loop{

    //     print!("Enter path to file:");
    //     std::io::stdout().flush();

    //     std::io::stdin().read_line(&mut file_path);

    //     file_path = file_path.trim().to_string();

    //     if Path::new(&file_path).exists(){
    //         println!("PATH EXISTS.");
    //         break;
    //     }
    //     println!("ERROR INVALID FILE PATH.");
    // }
    let file_path = FileDialog::new().set_directory("/".to_string()).pick_file();
    // let content = file.unwrap();

    let mut file = File::create(&file_path.unwrap()).await.unwrap();
    let content = File::read_to_string(&mut file,&mut buffer);
    println!("{}",content.await.unwrap());
}