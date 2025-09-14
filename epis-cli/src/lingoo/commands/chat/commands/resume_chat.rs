use anyhow::Result;
use colored::Colorize;

use crate::lingoo::{commands::chat::commands::utils::select_conversation, utils::chat_round};

pub async fn handle_lingoo_resume_chat() -> Result<()> {
  let selected_conversation = select_conversation().await?;

  // TODO: Show a brief history of past messages

  loop {
    let ai_reply = chat_round(selected_conversation.id.clone()).await?;

    println!("{} {}", "Epis >".bold().cyan(), ai_reply)
  }
}
