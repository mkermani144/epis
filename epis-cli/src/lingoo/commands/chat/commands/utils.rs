use anyhow::{Context, Result};
use inquire::Select;

use crate::{
  config::CONFIG,
  lingoo::{
    api::LingooHttpApi,
    commands::chat::commands::models::{PartialConversation, PartialConversationsList},
  },
};

pub async fn select_conversation() -> Result<PartialConversation> {
  let epis_host = &*CONFIG.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());

  let conversations = lingoo_api.list_conversations().await?;
  let partial_conversations = PartialConversationsList::from(conversations).into_vec();

  let selected_conversation = Select::new("select conversation", partial_conversations)
    .prompt()
    .context("error while parsing conversation to rename")?;

  Ok(selected_conversation)
}
