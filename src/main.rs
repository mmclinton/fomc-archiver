/*
TODO: do not allow the user to run 'make' without providing an API key (for now)
*/
#![allow(non_snake_case)]
use anyhow::Result;
use std::env;

mod api;
mod config;
mod db;
mod display;

#[tokio::main]
async fn main() -> Result<()> {
    let db = db::Database::initialize()?;

    let is_update = env::args().any(|arg| arg == "--update");

    if is_update {
        match db.update_database().await {
            Ok(()) => println!("Database updated successfully!"),
            Err(e) => eprintln!("Error updating database: {}", e),
        }
    } else {
        match display::fetch_and_print_videos(&db).await {
            Ok(()) => print!(""),
            Err(e) => eprintln!("Error fetching videos: {}", e),
        }
    }

    Ok(())
}
