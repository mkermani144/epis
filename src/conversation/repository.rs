use anyhow::Result;

use crate::{
  conversation::models::{
    CreateConversationRequest, GetConversationMessageHistoryRequest, SetConversationTitleRequest,
    StoreMessageRequest,
  },
  entities::common::{ChatMessage, Id},
};

pub trait ConversationRepository: Clone + Send + Sync + 'static {
  fn create_conversation(
    &self,
    request: &CreateConversationRequest,
  ) -> impl Future<Output = Result<Id>> + Send;
  fn set_conversation_title(
    &self,
    request: &SetConversationTitleRequest,
  ) -> impl Future<Output = Result<()>> + Send;
  fn store_message(&self, request: &StoreMessageRequest)
  -> impl Future<Output = Result<Id>> + Send;
  fn get_conversation_message_history(
    &self,
    request: &GetConversationMessageHistoryRequest,
  ) -> impl Future<Output = Result<Vec<ChatMessage>>> + Send;
}
