use anyhow::Result;

use crate::categorizer::categorizer::Category;
use crate::conversation::types::ConversationTitle;
use crate::types::common::{ChatMessage, Id};

use super::repository::ConversationRepository;

pub struct ConversationService<R: ConversationRepository> {
  repository: R,
}

impl<R: ConversationRepository> ConversationService<R> {
  pub fn new(repository: R) -> Self {
    Self { repository }
  }

  pub async fn initiate_conversation(&self, category: &Category) -> Result<Id> {
    self.repository.create_conversation(category).await
  }

  pub async fn store_message(
    &self,
    conversation_id: &Id,
    chat_message: &ChatMessage,
  ) -> Result<Id> {
    self
      .repository
      .insert_message(conversation_id, chat_message)
      .await
  }

  pub async fn set_conversation_title(
    &self,
    conversation_id: &Id,
    title: &ConversationTitle,
  ) -> Result<()> {
    self
      .repository
      .update_conversation_title(conversation_id, title)
      .await
  }
}
