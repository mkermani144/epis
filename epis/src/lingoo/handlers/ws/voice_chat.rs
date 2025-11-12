mod charge;
mod message;
mod session;
mod state;

use anyhow::bail;
use axum::{
  Extension,
  extract::{
    State,
    ws::{Message, WebSocket, WebSocketUpgrade},
  },
  response::Response,
};
use clerk_rs::validators::authorizer::ClerkJwt;
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;
use tracing::{debug, instrument, trace, warn};

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{message::VoiceChatReplyMessage, session::VoiceChatSession},
};

pub async fn voice_chat<L: Llm, CR: ConversationRepository, S: Stt, T: Tts>(
  ws: WebSocketUpgrade,
  State(app_state): State<LingooAppState<L, CR, S, T>>,
  Extension(jwt): Extension<ClerkJwt>,
) -> Response {
  ws.on_upgrade(|socket| handle_socket(socket, app_state, jwt))
}

#[instrument(skip_all)]
async fn handle_socket<L: Llm, CR: ConversationRepository, S: Stt, T: Tts>(
  mut socket: WebSocket,
  app_state: LingooAppState<L, CR, S, T>,
  jwt: ClerkJwt,
) {
  // Replay to client with a [VoiceChatReplyMessage]. Error is returned if the client is
  // disconnected.
  //
  // TODO: Maybe use a newtype to extract these as its methods
  let reply =
    async |socket: &mut WebSocket, message: &VoiceChatReplyMessage| -> Result<(), anyhow::Error> {
      let result = socket
        .send(Message::Text(
          serde_json::to_string(message)
            .expect("Server replies are serializable")
            .into(),
        ))
        .await;

      // If we cannot send, it means the other part is disconnected
      result.or_else(|_| bail!("Socket client side disconnected"))
    };

  let mut session = VoiceChatSession::new(app_state.clone(), jwt);

  while let Some(raw_message) = socket.recv().await {
    // While there is a message, try to parse it into a [VoiceChatMessage] and handle it through
    // session. If message is invalid, reply with a message indicating it.
    match raw_message {
      Ok(raw_message) => {
        match raw_message {
          Message::Text(raw_text_message) => {
            if let Ok(parsed_message) = serde_json::from_str(&raw_text_message) {
              debug!(message = %parsed_message, "Message received and parsed");
              let message_to_reply = session.handle_message(parsed_message).await;

              if let Err(_) = reply(&mut socket, &message_to_reply).await {
                debug!("Failed to reply to message");
                return;
              };
            } else {
              trace!("An invalid text message received");
              if let Err(_) = reply(&mut socket, &VoiceChatReplyMessage::Invalid).await {
                trace!("Failed to reply to message");
                return;
              }
            }
          }
          _ => {
            trace!("An invalid non-text message received");
            if let Err(_) = reply(&mut socket, &VoiceChatReplyMessage::Invalid).await {
              trace!("Failed to reply to message");
              return;
            }
          }
        };
      }
      Err(error) => {
        warn!(%error, "Failed to receive message");
        return;
      }
    }
  }
}
