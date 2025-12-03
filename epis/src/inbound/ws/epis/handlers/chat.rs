use std::{str::FromStr, sync::Arc};

use axum::{
  extract::{Path, Query, State, WebSocketUpgrade, ws::WebSocket},
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::{debug, instrument, warn};

use crate::{
  domain::{
    models::{EpisAudioMessageFormat, UserId},
    ports::{Epis, UserManagement},
  },
  entities::common::Id,
  inbound::http::AppStateV2,
};

#[derive(Debug, Clone, Deserialize)]
/// Query params of this route
pub struct VoiceChatQueryParams {
  /// Format of audio messages
  audio_format: String,
  /// JWT for authorization
  jwt: String,
}

/// voice chat ws handler
#[instrument(skip_all)]
pub async fn chat<E: Epis, UM: UserManagement>(
  ws: WebSocketUpgrade,
  State(app_state): State<AppStateV2<E, UM>>,
  Path(chatmate_id): Path<Id>,
  Query(query): Query<VoiceChatQueryParams>,
) -> Response {
  // TODO: Validate JWT, authorize by charge, and extract user id

  if let Ok(audio_format) = EpisAudioMessageFormat::from_str(&query.audio_format) {
    debug!(user_id="", %chatmate_id, audio_format=%query.audio_format, "Chat session started");

    return ws
      .on_upgrade(|socket| handle_socket(socket, app_state, "".into(), chatmate_id, audio_format));
  }

  debug!(user_id="", %chatmate_id, audio_format=%query.audio_format, "Chat session did not start because of invalid audio format");

  (
    StatusCode::BAD_REQUEST,
    "Invalid or unsupported audio format",
  )
    .into_response()
}

/// The socket handler for voice_chat handler
async fn handle_socket<E: Epis, UM: UserManagement>(
  socket: WebSocket,
  app_state: AppStateV2<E, UM>,
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
