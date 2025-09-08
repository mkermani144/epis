use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::lingoo::lingoo::{LingooCommand, handle_lingoo};

mod config;
mod lingoo;

#[derive(Debug, Subcommand)]
enum EpisCommand {
  Lingoo {
    #[command(subcommand)]
    command: LingooCommand,
  },
}

#[derive(Parser, Debug)]
struct Cli {
  #[command(subcommand)]
  command: EpisCommand,
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    EpisCommand::Lingoo { command } => handle_lingoo(command).await,
  }
}
