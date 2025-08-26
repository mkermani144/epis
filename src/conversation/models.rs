use nutype::nutype;

use crate::{
  categorizer::categorizer::Category,
  entities::common::{ChatMessage, Id},
};

#[nutype(derive(Debug, Clone, AsRef), validate(not_empty))]
pub struct ConversationTitle(String);

pub struct CreateConversationRequest {
  category: Category,
}
impl CreateConversationRequest {
  pub fn new(category: Category) -> Self {
    Self { category }
  }

  pub fn category(&self) -> &Category {
    &self.category
  }
}

pub struct SetConversationTitleRequest {
  conversation_id: Id,
  title: ConversationTitle,
}
impl SetConversationTitleRequest {
  pub fn new(conversation_id: Id, title: ConversationTitle) -> Self {
    Self {
      conversation_id,
      title,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn title(&self) -> &ConversationTitle {
    &self.title
  }
}

pub struct StoreMessageRequest {
  conversation_id: Id,
  message: ChatMessage,
}
impl StoreMessageRequest {
  pub fn new(conversation_id: Id, message: ChatMessage) -> Self {
    Self {
      conversation_id,
      message,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn message(&self) -> &ChatMessage {
    &self.message
  }
}

pub struct GetConversationMessageHistoryRequest {
  conversation_id: Id,
}
impl GetConversationMessageHistoryRequest {
  pub fn new(conversation_id: Id) -> Self {
    Self { conversation_id }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }
}
