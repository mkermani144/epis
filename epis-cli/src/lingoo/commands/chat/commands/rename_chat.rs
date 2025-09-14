use anyhow::Result;

use crate::{
  config::CONFIG,
  lingoo::{
    api::LingooHttpApi, commands::chat::commands::utils::select_conversation, utils::prompt,
  },
};

pub async fn handle_lingoo_rename_chat() -> Result<()> {
  let selected_conversation = select_conversation().await?;

  let new_title = prompt("new title".into())?;

  let epis_host = &*CONFIG.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());

  lingoo_api
    .set_conversation_title(selected_conversation.id, new_title)
    .await?;

  Ok(())
}
