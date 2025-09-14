use anyhow::Result;
use inquire::{Text, validator::Validation};

use crate::{config::CONFIG, lingoo::api::LingooHttpApi};

pub fn prompt(message: Option<&str>) -> Result<String> {
  Text::new(message.unwrap_or(""))
    .with_validator(|text: &str| {
      if text.is_empty() {
        Ok(Validation::Invalid("Please enter a non-empty input".into()))
      } else {
        Ok(Validation::Valid)
      }
    })
    .prompt()
    .map_err(|e| e.into())
}

pub async fn chat_round(cid: String) -> Result<String> {
  let epis_host = &*CONFIG.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());

  let user_message = prompt(None)?;

  let ai_reply = lingoo_api.chat(cid.to_string(), user_message).await?;

  Ok(ai_reply.into_response())
}
