use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use urlencoding::encode;

#[derive(Parser, Debug)]
#[command(version, author, about = "A simple CLI application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Dev {
        path: PathBuf,
        #[arg(long, help = "Open the path in Cursor")]
        cursor: bool,
        #[arg(long, help = "Open the path in VS Code")]
        code: bool,
    },
    Search {
        query: String,
    },
}

fn launch(program: &str, program_name: &str, path: &PathBuf) -> Result<()> {
    Command::new(program).arg(path).spawn().with_context(|| {
        format!(
            "Failed to launch {} with path {}",
            program_name,
            path.display()
        )
    })?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dev { path, cursor, code } => {
            let open_all = !cursor && !code;
            let mut errors: Vec<anyhow::Error> = Vec::new();
            let mut successes = 0_usize;

            if *cursor || open_all {
                match launch("cursor", "Cursor", path) {
                    Ok(_) => successes += 1,
                    Err(e) => errors.push(e),
                };
            }

            if *code || open_all {
                match launch("code", "VS Code", path) {
                    Ok(_) => successes += 1,
                    Err(e) => errors.push(e),
                };
            }

            if successes == 0 && !errors.is_empty() {
                let joined = errors
                    .iter()
                    .map(|e| format!("- {e:#}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow::bail!("All editor launch attempts failed:\n{joined}");
            }

            for e in &errors {
                eprintln!("warning: {e:#}");
            }
        }
        Commands::Search { query } => {
            let url = format!("https://www.google.com/search?q={}", encode(query));
            open::that(&url).with_context(|| format!("Failed to open browser for {url}"))?;
        }
    }

    Ok(())
}
