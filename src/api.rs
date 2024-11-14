use crate::config::read_config;

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3/search?part=snippet&channelId=UCAzhpt9DmG6PnHXjmJTvRGQ&q=Press+Conference&type=video&order=date&maxResults=50&key=";

pub fn api_url() -> Result<String, Box<dyn std::error::Error>> {
    let api_key = read_config()?;
    let joined_url = format!("{}{}", BASE_URL, api_key);
    Ok(joined_url)
}
