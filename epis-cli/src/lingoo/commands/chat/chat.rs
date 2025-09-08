use anyhow::Result;
use clap::Subcommand;

use crate::lingoo::commands::chat::commands::{
  new_chat::handle_lingoo_new_chat, resume_chat::handle_lingoo_resume_chat,
};

#[derive(Debug, Subcommand)]
pub enum LingooChatCommand {
  New,
  Resume,
}
pub async fn handle_lingoo_chat(command: LingooChatCommand) -> Result<()> {
  match command {
    LingooChatCommand::New => handle_lingoo_new_chat().await,
    LingooChatCommand::Resume => handle_lingoo_resume_chat().await,
  }
}
