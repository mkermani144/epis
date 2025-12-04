//! Chat handler

use std::{str::FromStr, sync::Arc};

use axum::{
  Extension,
  extract::{Path, Query, State, WebSocketUpgrade, ws::WebSocket},
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::{debug, instrument, warn};

use crate::{
  domain::{
    models::{EpisAudioMessageFormat, Id, User, UserId},
    ports::{Epis, UserManagement},
  },
  inbound::http::AppState,
};

#[derive(Debug, Clone, Deserialize)]
/// Query params of this route
pub struct VoiceChatQueryParams {
  /// Format of audio messages
  audio_format: Option<String>,
  /// JWT for authorization
  #[allow(dead_code)]
  jwt: String,
}

/// voice chat ws handler
#[instrument(skip_all)]
pub async fn chat<E: Epis, UM: UserManagement>(
  ws: WebSocketUpgrade,
  State(app_state): State<AppState<E, UM>>,
  Path(chatmate_id): Path<Id>,
  Extension(user): Extension<User>,
  Query(query): Query<VoiceChatQueryParams>,
) -> Response {
  let audio_format = query
    .audio_format
    .unwrap_or(EpisAudioMessageFormat::default().to_string());

  if let Ok(audio_format) = EpisAudioMessageFormat::from_str(&audio_format) {
    let user_id = user.id().to_string();

    debug!(%user_id, %chatmate_id, %audio_format, "Chat session started");

    return ws
      .on_upgrade(|socket| handle_socket(socket, app_state, user_id, chatmate_id, audio_format));
  }

  debug!(user_id="", %chatmate_id, %audio_format, "Chat session did not start because of invalid audio format");

  (
    StatusCode::BAD_REQUEST,
    "Invalid or unsupported audio format",
  )
    .into_response()
}

/// The socket handler for voice_chat handler
async fn handle_socket<E: Epis, UM: UserManagement>(
  socket: WebSocket,
  app_state: AppState<E, UM>,
  user_id: UserId,
  chatmate_id: Id,
  audio_format: EpisAudioMessageFormat,
) {
  let mut duplex = Arc::new(Mutex::new(socket));

  app_state
    .epis()
    .chat(&user_id, &chatmate_id, &mut duplex, &audio_format)
    .await
    .inspect_err(|error| warn!(%error, "Epis chat loop returned with an error"))
    .unwrap_or_default()
}
