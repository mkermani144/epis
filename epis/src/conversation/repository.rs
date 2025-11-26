use epis_core::non_empty_text::NonEmptyString;

use crate::{
  conversation::models::{
    Conversation, ConversationTitle, CreateConversationError, GetConversationMessageHistoryError,
    GetConversationUserIdError, ListConversationsError, SetConversationTitleError,
    StoreMessageError,
  },
  entities::common::{ChatMessage, Id},
};

pub trait ConversationRepository: Clone + Send + Sync + 'static {
  fn create_conversation(
    &self,
    user_id: &NonEmptyString,
  ) -> impl Future<Output = Result<Id, CreateConversationError>> + Send;
  fn list_conversations(
    &self,
    user_id: &NonEmptyString,
  ) -> impl Future<Output = Result<Vec<Conversation>, ListConversationsError>> + Send;
  fn set_conversation_title(
    &self,
    cid: &Id,
    title: &ConversationTitle,
  ) -> impl Future<Output = Result<(), SetConversationTitleError>> + Send;
  fn store_message(
    &self,
    cid: &Id,
    message: &ChatMessage,
  ) -> impl Future<Output = Result<Id, StoreMessageError>> + Send;
  fn get_conversation_message_history(
    &self,
    cid: &Id,
  ) -> impl Future<Output = Result<Vec<ChatMessage>, GetConversationMessageHistoryError>> + Send;
  fn get_conversation_user_id(
    &self,
    cid: &Id,
  ) -> impl Future<Output = Result<NonEmptyString, GetConversationUserIdError>> + Send;
}
