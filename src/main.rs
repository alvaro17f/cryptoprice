mod modules;
mod utils;

use crate::utils::{
    completions::{print_completions, set_completions},
    tools::clear,
};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use color_print::cprintln;
use modules::{get_top::get_top, search::search};
use std::process::exit;

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // If provided, generate completions for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    /// List of available commands
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Search for a coin
    Search { query: String },
    /// Show top 100 coins
    Top,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        if generator == Shell::Zsh || generator == Shell::Bash {
            set_completions(generator, &mut cmd);
            cprintln!("<c>{}</c> <y>completions are set", generator);
            exit(0)
        } else {
            print_completions(generator, &mut cmd);
            exit(0)
        }
    }
    match &cli.commands {
        Some(Commands::Search { query }) => {
            clear()?;
            search(query).await?;
        }
        Some(Commands::Top) => {
            clear()?;
            get_top().await?;
        }
        None => {
            clear()?;
            get_top().await?;
        }
    }

    Ok(())
}
