use anyhow::Result;
use sqlx::query;

use crate::{
  categorizer::categorizer::Category,
  conversation::{repository::ConversationRepository, types::ConversationTitle},
  postgres::Postgres,
  types::common::Id,
};

impl ConversationRepository for Postgres {
  async fn create_conversation(&self, category: &Category) -> Result<Id> {
    let conversation = query!(
      "INSERT INTO conversation (category) VALUES ($1) RETURNING id",
      category as _,
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

  async fn insert_message(
    &self,
    conversation_id: &Id,
    message: &crate::types::common::Message,
  ) -> Result<Id> {
    todo!()
  }
}
