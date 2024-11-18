use crate::db;
use anyhow::Result;

pub async fn fetch_and_print_videos(db: &db::Database) -> Result<()> {
    db.fetch_all_videos()
        .map(|videos| {
            let table_entries: Vec<String> = videos
                .into_iter()
                .flat_map(|(_, published_at, link)| {
                    vec![
                        format!("Date: {}", published_at),
                        format!("Link: {}", link),
                        String::new(),
                    ]
                })
                .collect();

            println!("{}", ascii_table(table_entries));
            Ok(())
        })
        .unwrap_or_else(|e| {
            Err(anyhow::anyhow!("Error fetching videos from the database: {}", e).into())
        })
}

fn ascii_table(entries: Vec<String>) -> String {
    if entries.is_empty() {
        return "No content to display.".into();
    }

    let max_length = entries.iter().map(|entry| entry.len()).max().unwrap_or(0);
    let border = format!("+{}+", "-".repeat(max_length + 4));

    let table = entries
        .into_iter()
        .fold(format!("{}\n", border), |acc, entry| {
            acc + &format!("| {:<width$} |\n", entry, width = max_length + 2)
        });

    table + &border
}
