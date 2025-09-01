use thiserror::Error;

use crate::{
  conversation::models::{GetConversationMessageHistoryError, StoreMessageError},
  entities::common::{Id, Message},
};

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
#[derive(Error, Debug)]
pub enum LingooChatError {
  #[error("Error while getting conversation message history")]
  GetConversationMessageHistory(#[from] GetConversationMessageHistoryError),
  #[error("Error getting a response from LLM")]
  Llm,
  #[error("Error while storing messages")]
  StoreMessage(#[from] StoreMessageError),
  #[error("unknown error during chat")]
  Unknown,
}
