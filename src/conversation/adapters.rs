use crate::{
  categorizer::categorizer::Category, conversation::repository::ConversationRepository,
  postgres::Postgres, types::common::Id,
};

impl ConversationRepository for Postgres {
  fn create_conversation(&self, category: &Category) -> Id {
    todo!()
  }

  fn update_conversation_title(
    &self,
    conversation_id: &Id,
    title: &super::types::ConversationTitle,
  ) {
    todo!()
  }

  fn insert_message(&self, conversation_id: &Id, message: &crate::types::common::Message) {
    todo!()
  }
}
