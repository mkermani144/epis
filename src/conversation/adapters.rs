use anyhow::Result;
use sqlx::query;

use crate::{
  conversation::{
    models::{
      Conversation, ConversationTitle, CreateConversationRequest,
      GetConversationMessageHistoryRequest, SetConversationTitleRequest, StoreMessageRequest,
      Timestamp,
    },
    repository::ConversationRepository,
  },
  entities::common::{Category, ChatMessage, ChatMessageRole, Id},
  postgres::Postgres,
};

impl ConversationRepository for Postgres {
  async fn create_conversation(&self, request: &CreateConversationRequest) -> Result<Id> {
    let category_str = match request.category() {
      Category::Languages => "languages",
      Category::Invalid => "invalid",
    };

    let conversation = query!(
      "INSERT INTO conversation (category) VALUES ($1) RETURNING id",
      category_str
    )
    .fetch_one(self.pool())
    .await?;

    Ok(conversation.id.into())
  }

  async fn list_conversations(&self) -> Result<Vec<Conversation>> {
    let all_conversations = query!("SELECT * FROM conversation")
      .fetch_all(self.pool())
      .await?;

    Ok(
      all_conversations
        .iter()
        .map(|conversation| {
          let id = Id::new(conversation.id);
          let title = conversation
            .title
            .as_ref()
            // TODO: Do not unwrap
            .map(|t| ConversationTitle::try_new(t).unwrap());
          let category = match conversation.category.as_str() {
            "languages" => Category::Languages,
            _ => Category::Invalid,
          };
          let created_at = Timestamp::new(conversation.created_at.unix_timestamp() as u64);
          let updated_at = Timestamp::new(conversation.updated_at.unix_timestamp() as u64);
          
          Conversation::new(id, title, category, created_at, updated_at)
        })
        .collect(),
    )
  }

  async fn set_conversation_title(&self, request: &SetConversationTitleRequest) -> Result<()> {
    query!(
      "UPDATE conversation SET title = $1 WHERE id = $2",
      request.title().as_ref(),
      request.conversation_id().as_ref(),
    )
    .execute(self.pool())
    .await?;

    Ok(())
  }

  async fn store_message(&self, request: &StoreMessageRequest) -> Result<Id> {
    let role = match request.message().role {
      ChatMessageRole::User => "user",
      ChatMessageRole::Ai => "ai",
      ChatMessageRole::System => "system",
    };

    let message = query!(
      "INSERT INTO message (conversation_id, content, role) VALUES ($1, $2, $3) RETURNING id",
      request.conversation_id().as_ref(),
      request.message().message.as_ref(),
      role,
    )
    .fetch_one(self.pool())
    .await?;

    Ok(message.id.into())
  }

  async fn get_conversation_message_history(
    &self,
    request: &GetConversationMessageHistoryRequest,
  ) -> Result<Vec<ChatMessage>> {
    let messages = query!(
      "SELECT id, content, role FROM message WHERE conversation_id = $1",
      request.conversation_id().as_ref(),
    )
    .fetch_all(self.pool())
    .await?;

    let message_history = messages
      .into_iter()
      .filter_map(|message| {
        let mut role = match message.role.as_str() {
          "user" => Some(ChatMessageRole::User),
          "ai" => Some(ChatMessageRole::Ai),
          "system" => Some(ChatMessageRole::System),
          _ => None,
        };

        Some(ChatMessage {
          role: role.take()?,
          message: message.content.into(),
        })
      })
      .collect();

    Ok(message_history)
  }
}
