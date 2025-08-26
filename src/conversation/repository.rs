use anyhow::Result;

use crate::{
  conversation::models::{
    CreateConversationRequest, SetConversationTitleRequest, StoreMessageRequest,
  },
  entities::common::Id,
};

pub trait ConversationRepository: Send + Sync + 'static {
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
}
