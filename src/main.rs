use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GithubStatusComponent {
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct GithubStatusComponents {
    components: Vec<GithubStatusComponent>,
}

fn print_status(name: &str, status: &str, color: Color) -> anyhow::Result<()> {
    execute!(
        stdout(),
        Print(format!("{:30}\t", name)),
        SetForegroundColor(color),
        Print(format!("{}\n", status)),
        ResetColor
    )?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let statuses = reqwest::get("https://www.githubstatus.com/api/v2/components.json")
        .await?
        .json::<GithubStatusComponents>()
        .await?;

    let filtered_statuses: Vec<&GithubStatusComponent> = statuses
        .components
        .iter()
        .filter(|status| {
            !status
                .name
                .to_lowercase()
                .contains("visit www.githubstatus.com")
        })
        .collect();

    for status in filtered_statuses {
        match status {
            GithubStatusComponent { name, status } if status == "operational" => {
                print_status(&name, "OK", Color::DarkGreen)?
            }
            GithubStatusComponent { name, status } if status == "degraded_performance" => {
                print_status(&name, "DEGRADED", Color::DarkYellow)?
            }
            GithubStatusComponent { name, status } if status == "partial_outage" => {
                print_status(&name, "OUTAGE", Color::Red)?
            }
            GithubStatusComponent { name, status } if status == "major_outage" => {
                print_status(&name, "DOWN", Color::DarkRed)?
            }
            GithubStatusComponent { name, status } => print_status(&name, &status, Color::Blue)?,
        }
    }

    Ok(())
}
