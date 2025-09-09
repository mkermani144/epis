use anyhow::{Context, Result};
use colored::Colorize;
use derive_more::Display;
use inquire::Select;

use crate::{
  config::config,
  lingoo::{api::LingooHttpApi, utils::chat_round},
};

#[derive(Display)]
#[display(
  "{}{}",
  self.title.as_ref().map_or("untitled", |title| title),
  if self.title.is_none() { format!(" ({id:.5}...)") } else { "".into() }
)]
struct PartialConversation {
  id: String,
  title: Option<String>,
}

pub async fn handle_lingoo_resume_chat() -> Result<()> {
  let epis_host = &*config.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());
  let conversations = lingoo_api.list_conversations().await?;

  let conversation_titles = conversations
    .data()
    .into_iter()
    .map(|conversation| PartialConversation {
      id: (conversation.id().to_string()),
      title: conversation.title(),
    })
    .collect();

  let selected_conversation = Select::new("select conversation", conversation_titles)
    .prompt()
    .context("error while parsing conversation to resume")?;

  // TODO: Show a brief history of past messages

  loop {
    let ai_reply = chat_round(selected_conversation.id.clone()).await?;

    println!("{} {}", "Epis >".bold().cyan(), ai_reply)
  }
}
