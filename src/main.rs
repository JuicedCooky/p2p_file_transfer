use std::error::Error;
use std::io::Write;

mod host;
mod client;
mod utils;
mod dual;
mod thread;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    loop {

        clearscreen::clear().expect("failed to clear screen");

        println!("1. Be host device.");
        println!("2. Be client to connect to host.");
        println!("3. Be dual host and client");
        println!("4. Quit application.");
        print!("Enter choice:");
        std::io::stdout().flush().unwrap();

        let choice = dual::take_input();

        match choice.as_str() {
            "1" => {
                let _host = host::Host::new().await?;
            }
            "2" => {
                let _client = client::Client::new().await?;
            }
            "3" => {
                let _dual = dual::Dual::new().await?;
            }
            "4" => break,
            _ => continue
        }
      
    }

    Ok(())
}