use sqlx::{Error::RowNotFound, query};

use crate::{
  conversation::{
    models::{
      Conversation, ConversationTitle, CreateConversationError, CreateConversationRequest,
      GetConversationMessageHistoryError, GetConversationMessageHistoryRequest,
      ListConversationsError, SetConversationTitleError, SetConversationTitleRequest,
      StoreMessageError, StoreMessageRequest, Timestamp,
    },
    repository::ConversationRepository,
  },
  entities::common::{Category, ChatMessage, ChatMessageRole, Id},
  postgres::Postgres,
};

impl ConversationRepository for Postgres {
  async fn create_conversation(
    &self,
    request: &CreateConversationRequest,
  ) -> Result<Id, CreateConversationError> {
    let category_str = match request.category() {
      Category::Languages => "languages",
      Category::Invalid => "invalid",
    };

    let conversation = query!(
      "INSERT INTO conversation (category) VALUES ($1) RETURNING id",
      category_str
    )
    .fetch_one(self.pool())
    .await
    .map_err(|_| CreateConversationError::Unknown)?;

    Ok(conversation.id.into())
  }

  async fn list_conversations(&self) -> Result<Vec<Conversation>, ListConversationsError> {
    let all_conversations = query!("SELECT * FROM conversation")
      .fetch_all(self.pool())
      .await
      .map_err(|_| ListConversationsError::Unknown)?;

    Ok(
      all_conversations
        .iter()
        .map(|conversation| {
          let id = Id::new(conversation.id);
          let title = conversation
            .title
            .as_ref()
            // TODO: Do not unwrap. Or maybe should we?
            .map(|t| {
              ConversationTitle::try_new(t).expect("Stored conversation titles are never empty")
            });
          let category = match conversation.category.as_str() {
            "languages" => Category::Languages,
            _ => Category::Invalid,
          };
          let created_at = Timestamp::new(conversation.created_at.unix_timestamp() as u64);
          let updated_at = Timestamp::new(conversation.updated_at.unix_timestamp() as u64);

          Conversation::new(id, title, category, created_at, updated_at)
        })
        .collect(),
    )
  }

  async fn set_conversation_title(
    &self,
    request: &SetConversationTitleRequest,
  ) -> Result<(), SetConversationTitleError> {
    query!(
      "UPDATE conversation SET title = $1 WHERE id = $2 RETURNING id",
      request.title().as_ref(),
      request.conversation_id().as_ref(),
    )
    .fetch_one(self.pool())
    .await
    .map_err(|e| match e {
      RowNotFound => SetConversationTitleError::NotFoundConversation,
      _ => SetConversationTitleError::Unknown,
    })?;

    Ok(())
  }

  async fn store_message(&self, request: &StoreMessageRequest) -> Result<Id, StoreMessageError> {
    query!(
      "SELECT * FROM conversation WHERE id = $1",
      request.conversation_id().as_ref()
    )
    .fetch_one(self.pool())
    .await
    .map_err(|e| match e {
      RowNotFound => StoreMessageError::NotFoundConversation,
      _ => StoreMessageError::Unknown,
    })?;

    let role = match request.message().role {
      ChatMessageRole::User => "user",
      ChatMessageRole::Ai => "ai",
      ChatMessageRole::System => "system",
    };

    let message = query!(
      "INSERT INTO message (conversation_id, content, role) VALUES ($1, $2, $3) RETURNING id",
      request.conversation_id().as_ref(),
      request.message().message.as_ref(),
      role,
    )
    .fetch_one(self.pool())
    .await
    .map_err(|e| match e {
      _ => StoreMessageError::Unknown,
    })?;

    Ok(message.id.into())
  }

  async fn get_conversation_message_history(
    &self,
    request: &GetConversationMessageHistoryRequest,
  ) -> Result<Vec<ChatMessage>, GetConversationMessageHistoryError> {
    query!(
      "SELECT * FROM conversation WHERE id = $1",
      request.conversation_id().as_ref()
    )
    .fetch_one(self.pool())
    .await
    .map_err(|e| match e {
      RowNotFound => GetConversationMessageHistoryError::NotFoundConversation,
      _ => GetConversationMessageHistoryError::Unknown,
    })?;

    let messages = query!(
      "SELECT id, content, role FROM message WHERE conversation_id = $1",
      request.conversation_id().as_ref(),
    )
    .fetch_all(self.pool())
    .await
    .map_err(|_| GetConversationMessageHistoryError::Unknown)?;

    let message_history = messages
      .into_iter()
      .filter_map(|message| {
        let mut role = match message.role.as_str() {
          "user" => Some(ChatMessageRole::User),
          "ai" => Some(ChatMessageRole::Ai),
          "system" => Some(ChatMessageRole::System),
          _ => None,
        };

        Some(ChatMessage {
          role: role.take()?,
          message: message.content.try_into().ok()?,
        })
      })
      .collect();

    Ok(message_history)
  }
}
