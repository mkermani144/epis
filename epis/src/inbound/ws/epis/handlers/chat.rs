use std::sync::{Arc, Mutex};

use axum::{
  extract::{Path, Query, State, WebSocketUpgrade, ws::WebSocket},
  response::Response,
};
use serde::Deserialize;
use tracing::warn;

use crate::{
  domain::{models::UserId, ports::Epis},
  entities::common::Id,
  inbound::http::AppStateV2,
};

#[derive(Debug, Clone, Deserialize)]
/// Query params of this route
pub struct VoiceChatQueryParams {
  /// JWT for authorization
  jwt: String,
}

/// voice chat ws handler
pub async fn chat<E: Epis>(
  ws: WebSocketUpgrade,
  State(app_state): State<AppStateV2<E>>,
  Path(chatmate_id): Path<Id>,
  Query(query): Query<VoiceChatQueryParams>,
) -> Response {
  // TODO: Validate JWT, authorize by charge, and extract user id

  ws.on_upgrade(|socket| handle_socket(socket, app_state, "".into(), chatmate_id))
}

/// The socket handler for voice_chat handler
async fn handle_socket<E: Epis>(
  socket: WebSocket,
  app_state: AppStateV2<E>,
  user_id: UserId,
  chatmate_id: Id,
) {
  let mut duplex = Arc::new(Mutex::new(socket));

  app_state
    .epis()
    .chat(&user_id, &chatmate_id, &mut duplex)
    .await
    .inspect_err(|error| warn!(%error, "Epis chat loop returned with an error"))
    .unwrap_or_default()
}
