use anyhow::Result;
use axum::{
  Router,
  extract::State,
  response::{IntoResponse, Json},
  routing::post,
};
use serde::{Deserialize, Serialize};

use crate::{
  conversation::repository::ConversationRepository,
  entities::common::{Id, Message},
  http::server::AppState,
  lingoo::models::LingooChatRequest,
  providers::llm::Llm,
};

#[derive(Debug, Serialize)]
pub struct CreateLingooConversationResponseData {
  conversation_id: String,
}
impl CreateLingooConversationResponseData {
  pub fn new(conversation_id: Id) -> Self {
    Self {
      conversation_id: conversation_id.to_string(),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct LingooChatRequestBody {
  conversation_id: String,
  message: String,
}
impl LingooChatRequestBody {
  pub fn try_into_domain(self) -> Result<LingooChatRequest> {
    Ok(LingooChatRequest::new(
      Id::try_from(self.conversation_id)?,
      Message::new(self.message),
    ))
  }
}

#[derive(Debug, Serialize)]
pub struct LingooChatResponseData {
  response: String,
}
impl LingooChatResponseData {
  pub fn new(response: Message) -> Self {
    Self {
      response: response.into_inner(),
    }
  }
}

#[derive(Debug)]
pub struct LingooRouter<L: Llm, R: ConversationRepository>(Router<AppState<L, R>>);

impl<L: Llm, R: ConversationRepository> LingooRouter<L, R> {
  pub fn new() -> Self {
    let router = Router::new()
      .route("/create", post(Self::create_conversation))
      .route("/chat", post(Self::chat));

    Self(router)
  }

  pub fn into_inner(self) -> axum::Router<AppState<L, R>> {
    self.0
  }

  async fn create_conversation(State(app_state): State<AppState<L, R>>) -> impl IntoResponse {
    let conversation_id = app_state.lingoo.create_conversation().await.unwrap();
    Json(CreateLingooConversationResponseData::new(conversation_id))
  }

  async fn chat(
    State(app_state): State<AppState<L, R>>,
    Json(request): Json<LingooChatRequestBody>,
  ) -> impl IntoResponse {
    let lingoo_chat_request = request.try_into_domain().unwrap();
    let message = app_state.lingoo.chat(&lingoo_chat_request).await.unwrap();
    Json(LingooChatResponseData::new(message))
  }
}
