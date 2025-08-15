use anyhow::Result;
use sqlx::query;

use crate::{
  categorizer::categorizer::Category,
  conversation::{repository::ConversationRepository, types::ConversationTitle},
  postgres::Postgres,
  types::common::{ChatMessage, ChatMessageRole, Id},
};

impl ConversationRepository for Postgres {
  async fn create_conversation(&self, category: &Category) -> Result<Id> {
    let category_str = match category {
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

  async fn update_conversation_title(
    &self,
    conversation_id: &Id,
    title: &ConversationTitle,
  ) -> Result<()> {
    query!(
      "UPDATE conversation SET title = $1 WHERE id = $2",
      title.as_ref(),
      conversation_id.as_ref(),
    )
    .execute(self.pool())
    .await?;

    Ok(())
  }

  async fn insert_message(&self, conversation_id: &Id, chat_message: &ChatMessage) -> Result<Id> {
    let role = match chat_message.role {
      ChatMessageRole::User => "user",
      ChatMessageRole::Ai => "ai",
      ChatMessageRole::System => "system",
    };

    let message = query!(
      "INSERT INTO message (conversation_id, content, role) VALUES ($1, $2, $3) RETURNING id",
      conversation_id.as_ref(),
      chat_message.message.as_ref(),
      role,
    )
    .fetch_one(self.pool())
    .await?;

    Ok(message.id.into())
  }
}
