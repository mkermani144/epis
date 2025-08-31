use crate::{
  conversation::models::{
    Conversation, CreateConversationRequest,
    GetConversationMessageHistoryRequest, SetConversationTitleError, SetConversationTitleRequest,
    StoreMessageRequest,
  },
  entities::common::{ChatMessage, Id},
};

pub trait ConversationRepository: Clone + Send + Sync + 'static {
  fn create_conversation(
    &self,
    request: &CreateConversationRequest,
  ) -> impl Future<Output = anyhow::Result<Id>> + Send;
  fn list_conversations(&self) -> impl Future<Output = anyhow::Result<Vec<Conversation>>> + Send;
  fn set_conversation_title(
    &self,
    request: &SetConversationTitleRequest,
  ) -> impl Future<Output = Result<(), SetConversationTitleError>> + Send;
  fn store_message(
    &self,
    request: &StoreMessageRequest,
  ) -> impl Future<Output = anyhow::Result<Id>> + Send;
  fn get_conversation_message_history(
    &self,
    request: &GetConversationMessageHistoryRequest,
  ) -> impl Future<Output = anyhow::Result<Vec<ChatMessage>>> + Send;
}
