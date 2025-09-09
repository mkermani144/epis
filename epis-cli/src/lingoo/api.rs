use std::collections::HashMap;

use anyhow::{Context, Result};
use epis::lingoo::handlers::{
  chat::{LingooChatRequestBody, LingooChatResponseData},
  create_conversation::CreateLingooConversationResponseData,
  list_conversations::ListLingooConversationsResponseData,
};

pub struct LingooHttpApi {
  base_url: String,
}

impl LingooHttpApi {
  pub fn new(base_url: String) -> Self {
    Self { base_url }
  }

  pub async fn list_conversations(&self) -> Result<ListLingooConversationsResponseData> {
    // TODO: Use Url crate or similar for handling url creation logic
    Ok(
      reqwest::get(format!("{}/lingoo/conversation/list", self.base_url))
        .await
        .context("failed to get lingoo conversations list")?
        .json()
        .await
        .context("failed to deserialize conversations list into a json")?,
    )
  }

  pub async fn chat(&self, cid: String, message: String) -> Result<LingooChatResponseData> {
    let body = LingooChatRequestBody::new(cid, message);

    Ok(
      reqwest::Client::new()
        .post(format!("{}/lingoo/chat", self.base_url))
        .json(&body)
        .send()
        .await
        .context("failed to post to lingoo chat endpoint")?
        .json()
        .await
        .context("failed to deserialize chat response into a json")?,
    )
  }

  pub async fn create_conversation(&self) -> Result<CreateLingooConversationResponseData> {
    Ok(
      reqwest::Client::new()
        .post(format!("{}/lingoo/conversation/create", self.base_url))
        .send()
        .await
        .context("failed to post to lingoo conversation create endpoint")?
        .json()
        .await
        .context("failed to deserialize created conversation response into a json")?,
    )
  }
}
