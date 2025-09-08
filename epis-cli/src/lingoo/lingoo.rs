use anyhow::Result;
use clap::Subcommand;

use crate::lingoo::commands::chat::chat::{LingooChatCommand, handle_lingoo_chat};

#[derive(Debug, Subcommand)]
pub enum LingooCommand {
  Chat {
    #[command(subcommand)]
    command: LingooChatCommand,
  },
}
pub async fn handle_lingoo(command: LingooCommand) -> Result<()> {
  match command {
    LingooCommand::Chat {
      command: subcommand,
    } => handle_lingoo_chat(subcommand).await,
  }
}
