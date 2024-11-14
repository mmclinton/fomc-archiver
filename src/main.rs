use config::{config_path, read_config};

mod config;
mod api;

fn main () {
    println!("The configuration file is located at {:?}", config_path());
    println!("The API key is {}", read_config().expect("Failed to read the API key"));
    println!("{}", api::api_url().expect("Failed to build the URL"));
}