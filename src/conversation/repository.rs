use crate::{
  conversation::models::{
    Conversation, CreateConversationError,
    GetConversationMessageHistoryError,
    ListConversationsError, SetConversationTitleError, SetConversationTitleRequest,
    StoreMessageError, StoreMessageRequest,
  },
  entities::common::{Category, ChatMessage, Id},
};

pub trait ConversationRepository: Clone + Send + Sync + 'static {
  fn create_conversation(
    &self,
    category: &Category,
  ) -> impl Future<Output = Result<Id, CreateConversationError>> + Send;
  fn list_conversations(
    &self,
  ) -> impl Future<Output = Result<Vec<Conversation>, ListConversationsError>> + Send;
  fn set_conversation_title(
    &self,
    request: &SetConversationTitleRequest,
  ) -> impl Future<Output = Result<(), SetConversationTitleError>> + Send;
  fn store_message(
    &self,
    request: &StoreMessageRequest,
  ) -> impl Future<Output = Result<Id, StoreMessageError>> + Send;
  fn get_conversation_message_history(
    &self,
    cid: &Id,
  ) -> impl Future<Output = Result<Vec<ChatMessage>, GetConversationMessageHistoryError>> + Send;
}
