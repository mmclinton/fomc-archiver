/*
TODO: finish makefile install logic
TODO: add a database and corresponding logic
TODO: do not allow the user to run 'make' without providing an API key (for now)
*/

mod api;
mod config;

#[tokio::main]

async fn main() {
    match api::YouTubeResponse::get_data(None).await {
        Ok(data) => {
            let filtered_items = data.filter();

            for item in filtered_items {
                println!(
                    "Title: {}\nDate: {}\n",
                    item.snippet.title, item.snippet.publishedAt
                );
            }

            let mut next_page_token = data.nextPageToken;

            while let Some(token) = next_page_token {
                match api::YouTubeResponse::get_data(Some(token)).await {
                    Ok(next_data) => {
                        let filtered_items = next_data.filter();
                        for item in filtered_items {
                            println!(
                                "Title: {}\nDate: {}\n",
                                item.snippet.title, item.snippet.publishedAt
                            );
                        }
                        next_page_token = next_data.nextPageToken;
                    }
                    Err(e) => {
                        eprintln!("Error fetching next page: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
