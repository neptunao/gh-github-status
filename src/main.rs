use std::io::Write;

use serde::Deserialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(None))?;
    write!(&mut stdout, "{:30}\t", name)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", status)?;
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
                print_status(&name, "OK", Color::Green)?
            }
            GithubStatusComponent { name, status } if status == "degraded_performance" => {
                print_status(&name, "DEGRADED", Color::Yellow)?
            }
            GithubStatusComponent { name, status } if status == "partial_outage" => {
                print_status(&name, "OUTAGE", Color::Red)?
            }
            GithubStatusComponent { name, status } if status == "major_outage" => {
                print_status(&name, "DOWN", Color::Red)?
            }
            GithubStatusComponent { name, status } => print_status(&name, &status, Color::Blue)?,
        }
    }

    Ok(())
}
