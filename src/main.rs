/*
TODO: add the ability to download the videos using the link within the db
*/
#![allow(non_snake_case)]
use crate::db::Database;
use anyhow::Result;
use clap::builder::ValueParser;
use clap::Parser;

mod api;
mod config;
mod db;
mod display;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Update the database")]
    update: bool,

    #[arg(long, short, value_parser = ValueParser::new(parse_limit), default_value_t = 11222, help = "Limit the number of videos to fetch")]
    limit: i64,
}

fn parse_limit(s: &str) -> Result<i64, String> {
    s.parse().map_err(|_| format!("Invalid limit value: {}", s))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let db = Database::initialize()?;

    if args.update {
        match db.update_database() {
            Ok(()) => println!("Database updated successfully!"),
            Err(e) => eprintln!("Error updating database: {}", e),
        }
    } else {
        match display::fetch_and_print_videos(&db, args.limit).await {
            Ok(()) => println!(""),
            Err(e) => eprintln!("Error fetching videos: {}", e),
        }
    }

    Ok(())
}
