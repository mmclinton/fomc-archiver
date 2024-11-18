/*
TODO: finish makefile install logic
TODO: add a database and corresponding logic
TODO: do not allow the user to run 'make' without providing an API key (for now)
*/
use anyhow::Result; 
use std::env;
mod api;
mod config;
mod db;

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
        match fetch_and_print_videos(&db).await {
            Ok(()) => println!("Videos fetched successfully!"),
            Err(e) => eprintln!("Error fetching videos: {}", e),
        }
    }

    Ok(())
}

async fn fetch_and_print_videos(db: &db::Database) -> Result<()> {
    match db.fetch_all_videos() {
        Ok(videos) => {
            println!("Videos in the database:\n");
            for (title, published_at, link) in videos {
                println!(
                    "Title: {}\nPublished At: {}\nLink: {}\n",
                    title, published_at, link
                );
            }
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("Error fetching videos from the database: {}", e).into()),
    }
}
