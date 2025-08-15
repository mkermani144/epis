use anyhow::Result;

use crate::categorizer::categorizer::Category;
use crate::conversation::types::ConversationTitle;
use crate::types::common::{Id, Message};

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

  pub async fn store_message(&self, conversation_id: &Id, message: &Message) -> Result<Id> {
    self
      .repository
      .insert_message(conversation_id, message)
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
