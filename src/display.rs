use crate::db;
use anyhow::Result;

pub async fn fetch_and_print_videos(db: &db::Database, limit: i64) -> Result<()> {
    let videos = db.fetch_n_videos(limit)?;

    if videos.is_empty() {
        println!("No videos found.");
    } else {
        let table_entries: Vec<(String, bool)> = videos
            .into_iter()
            .enumerate()
            .flat_map(|(i, (_, published_at, link))| {
                let is_cyan = i % 2 == 1;
                vec![(format!("{}: {}", published_at, link), is_cyan)]
            })
            .collect();

        println!("{}", ascii_table(table_entries));
    }

    Ok(())
}

fn ascii_table(entries: Vec<(String, bool)>) -> String {
    if entries.is_empty() {
        return "No content to display.".into();
    }

    let max_length = entries
        .iter()
        .map(|(entry, _)| entry.len())
        .max()
        .unwrap_or(0);
    let border = format!("+{}+", "-".repeat(max_length + 4));

    let table = entries
        .into_iter()
        .fold(format!("{}\n", border), |acc, (entry, is_cyan)| {
            let color = if is_cyan { "\x1b[36m" } else { "\x1b[0m" };
            acc + &format!(
                "|  {}{:<width$}\x1b[0m |\n",
                color,
                entry,
                width = max_length + 1
            )
        });

    table + &border
}
