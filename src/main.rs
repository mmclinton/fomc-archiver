use clap::Parser;
use reqwest;
use serde::Deserialize;
use std::error::Error;

const FOMC_LINK: &str = "https://www.federalreserve.gov/feeds/videos.xml";

#[derive(Debug, Deserialize)]
struct RSS {
    channel: Channel,
}

#[derive(Debug, Deserialize)]
struct Channel {
    // title: String,
    item: Vec<Video>,
}

#[derive(Debug, Deserialize)]
struct Video {
    title: String,
    link: String,
    #[serde(rename = "pubDate")]
    pub_date: String,
}

#[derive(Parser)]
struct Cli {
    /// Filter the default results to by any case-sensitive <string> provided.
    #[arg(short, long)]
    filter: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let response = reqwest::get(FOMC_LINK).await?;
    let data = response.text().await?;

    let rss: RSS = quick_xml::de::from_str(&data)?;

    for item in rss.channel.item {
        if let Some(filter) = &args.filter {
            if !item.title.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
        }

        if item.title.contains("FOMC Press Conference") {
            println!("\x1b[36mItem Title:\x1b[0m {}", item.title);
            println!("\x1b[36mDate:\x1b[0m {}", item.pub_date);
            println!("\x1b[36mLink:\x1b[0m {}", item.link);
            println!();
        }
    }

    Ok(())
}
