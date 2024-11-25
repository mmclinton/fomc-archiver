use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn initialize() -> Result<Self> {
        let db_path = Self::get_db_path();

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).context("Failed to create directories for database")?;
        }

        let conn = Connection::open(db_path).context("Failed to open database connection")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS videos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                published_at TEXT NOT NULL,
                link TEXT NOT NULL
            )",
            [],
        )
        .context("Failed to create videos table")?;

        Ok(Self { conn })
    }

    pub fn insert_video(&self, title: &str, published_at: &str, link: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO videos (title, published_at, link) VALUES (?1, ?2, ?3)",
                params![title, published_at, link],
            )
            .context("Failed to insert video into database")?;
        Ok(())
    }

    pub fn fetch_all_videos(&self) -> Result<Vec<(String, String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT title, published_at, link FROM videos")
            .context("Failed to prepare statement to fetch videos")?;
        let video_iter = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .context("Failed to map rows from video query")?;

        let mut videos = Vec::new();
        for video in video_iter {
            videos.push(video.context("Error reading video from database")?);
        }
        Ok(videos)
    }

    pub async fn update_database(&self) -> Result<()> {
        match crate::api::YouTubeResponse::get_data(None) {
            Ok(data) => {
                let filtered_items = data.filter();

                for item in filtered_items {
                    let formatted_date = format_date(&item.snippet.publishedAt);
                    let link = crate::api::YouTubeResponse::make_link(&item.id.videoId);

                    self.insert_video(&item.snippet.title, &formatted_date, &link)
                        .context("Failed to insert video")?;
                }

                let mut next_page_token = data.nextPageToken;
                while let Some(token) = next_page_token {
                    match crate::api::YouTubeResponse::get_data(Some(token)) {
                        Ok(next_data) => {
                            let filtered_items = next_data.filter();
                            for item in filtered_items {
                                let formatted_date = format_date(&item.snippet.publishedAt);
                                let link = crate::api::YouTubeResponse::make_link(&item.id.videoId);

                                self.insert_video(&item.snippet.title, &formatted_date, &link)
                                    .context("Failed to insert video")?;
                            }
                            next_page_token = next_data.nextPageToken;
                        }
                        Err(e) => {
                            eprintln!("Error fetching next page: {}", e);
                            break;
                        }
                    }
                }
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Error occurred while fetching data: {}", e)),
        }
    }

    fn get_db_path() -> PathBuf {
        let home_dir = env::var("HOME").expect("HOME environment variable not set");
        let mut path = PathBuf::from(home_dir);
        path.push(".local");
        path.push("share");
        path.push("fomc");
        path.push("fomc.db");
        path
    }
}

fn format_date(date: &str) -> String {
    let date_parts: Vec<&str> = date.split('T').collect();
    let date = date_parts[0];
    date.to_string()
}
