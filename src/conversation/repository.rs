use anyhow::Result;

use crate::{
  categorizer::categorizer::Category,
  conversation::types::ConversationTitle,
  types::common::{Id, Message},
};

pub trait ConversationRepository {
  async fn create_conversation(&self, category: &Category) -> Result<Id>;
  async fn update_conversation_title(
    &self,
    conversation_id: &Id,
    title: &ConversationTitle,
  ) -> Result<()>;
  async fn insert_message(&self, conversation_id: &Id, message: &Message) -> Result<Id>;
}
