use crate::db;
use anyhow::Result;

pub async fn fetch_and_print_videos(db: &db::Database) -> Result<()> {
    db.fetch_all_videos()
        .map(|videos| {
            let table_entries: Vec<(String, bool)> = videos
                .into_iter()
                .enumerate()
                .flat_map(|(i, (_, published_at, link))| {
                    let is_cyan = i % 2 == 1; 
                    vec![
                        (format!("{}: {}", published_at, link), is_cyan),
                        // (String::new(), false),
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

fn ascii_table(entries: Vec<(String, bool)>) -> String {
    if entries.is_empty() {
        return "No content to display.".into();
    }

    let max_length = entries.iter().map(|(entry, _)| entry.len()).max().unwrap_or(0);
    let border = format!("+{}+", "-".repeat(max_length + 5));

    let table = entries.into_iter().fold(format!("{}\n", border), |acc, (entry, is_cyan)| {
        let color = if is_cyan { "\x1b[36m" } else { "\x1b[0m" }; 
        acc + &format!(
            "|  {}{:<width$}\x1b[0m |\n",
            color,
            entry,
            width = max_length + 2
        )
    });

    table + &border
}
