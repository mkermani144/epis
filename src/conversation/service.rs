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

  pub fn initiate_conversation(&self, category: &Category) -> Id {
    self.repository.create_conversation(category)
  }

  pub fn store_message(&self, conversation_id: &Id, message: &Message) {
    self.repository.insert_message(conversation_id, message)
  }

  pub fn set_conversation_title(&self, conversation_id: &Id, title: &ConversationTitle) {
    self
      .repository
      .update_conversation_title(conversation_id, title);
  }
}
