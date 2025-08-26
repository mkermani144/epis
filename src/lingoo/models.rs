use crate::entities::common::{Id, Message};

pub struct LingooChatRequest {
  conversation_id: Id,
  message: Message,
}
impl LingooChatRequest {
  pub fn new(conversation_id: Id, message: Message) -> Self {
    Self {
      conversation_id,
      message,
    }
  }

  pub fn conversation_id(&self) -> &Id {
    &self.conversation_id
  }

  pub fn message(&self) -> &Message {
    &self.message
  }
}
