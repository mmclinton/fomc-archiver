use crate::config::read_config;
use std::error::Error;

const API_URL: &str = "https://www.googleapis.com/youtube/v3/search?part=snippet&channelId=UCAzhpt9DmG6PnHXjmJTvRGQ&q=Press+Conference&type=video&order=date&maxResults=50&key=";
const YOUTUBE_URL: &str = "https://www.youtube.com/watch?v=";

use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct YouTubeResponse {
    pub kind: String,
    pub etag: String,
    pub nextPageToken: Option<String>,
    pub regionCode: String,
    pub pageInfo: PageInfo,
    pub items: Vec<Item>,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub totalResults: u32,
    pub resultsPerPage: u32,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Item {
    pub kind: String,
    pub etag: String,
    pub id: Id,
    pub snippet: Snippet,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Id {
    pub kind: String,
    pub videoId: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Snippet {
    pub publishedAt: String,
    pub channelId: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub channelTitle: String,
    pub liveBroadcastContent: String,
    pub publishTime: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub medium: Thumbnail,
    pub high: Thumbnail,
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl YouTubeResponse {
    fn make_url(page_token: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = read_config()?;
        let mut url = format!("{}{}", API_URL, api_key);

        if let Some(token) = page_token {
            url.push_str(&format!("&pageToken={}", token))
        }
        Ok(url)
    }

    pub fn make_link(video_id: &str) -> String {
        format!("{}{}", YOUTUBE_URL, video_id)
    }

    pub async fn get_data(page_token: Option<String>) -> Result<YouTubeResponse, Box<dyn Error>> {
        let url = YouTubeResponse::make_url(page_token)?;
        let response = reqwest::get(url).await?;
        let json_data: YouTubeResponse = response.json().await?;
        Ok(json_data)
    }

    pub fn filter(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| {
                let title_lowercase = item.snippet.title.to_lowercase();
                title_lowercase.contains("press conference")
                    && !title_lowercase.contains("introductory")
                    && !title_lowercase.contains("#shorts")
            })
            .collect()
    }
}
