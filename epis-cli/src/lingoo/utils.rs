use anyhow::Result;
use inquire::Text;

use crate::{config::CONFIG, lingoo::api::LingooHttpApi};

pub async fn chat_round(cid: String) -> Result<String> {
  let epis_host = &*CONFIG.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());

  let user_message = Text::new("").prompt()?;

  let ai_reply = lingoo_api.chat(cid.to_string(), user_message).await?;

  Ok(ai_reply.into_response())
}
