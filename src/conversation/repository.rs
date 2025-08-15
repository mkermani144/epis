use anyhow::Result;

use crate::{
  categorizer::categorizer::Category,
  conversation::types::ConversationTitle,
  types::common::{ChatMessage, Id},
};

pub trait ConversationRepository {
  async fn create_conversation(&self, category: &Category) -> Result<Id>;
  async fn update_conversation_title(
    &self,
    conversation_id: &Id,
    title: &ConversationTitle,
  ) -> Result<()>;
  async fn insert_message(&self, conversation_id: &Id, chat_message: &ChatMessage) -> Result<Id>;
}
