use crate::{
  categorizer::categorizer::Category,
  conversation::types::ConversationTitle,
  types::common::{Id, Message},
};

pub trait ConversationRepository {
  fn create_conversation(&self, category: &Category) -> Id;
  fn update_conversation_title(&self, conversation_id: &Id, title: &ConversationTitle);
  fn insert_message(&self, conversation_id: &Id, message: &Message);
}
