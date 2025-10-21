mod message;
mod session;
mod state;

use anyhow::bail;
use axum::{
  extract::{
    State,
    ws::{Message, WebSocket, WebSocketUpgrade},
  },
  response::Response,
};
use epis_stt::stt::Stt;
use epis_tts::tts::Tts;

use crate::{
  ai::llm::Llm,
  conversation::repository::ConversationRepository,
  http::server::LingooAppState,
  lingoo::handlers::ws::voice_chat::{message::VoiceChatReplyMessage, session::VoiceChatSession},
  rag::rag::Rag,
};

pub async fn voice_chat<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  ws: WebSocketUpgrade,
  State(app_state): State<LingooAppState<L, CR, R, S, T>>,
) -> Response {
  ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

// FIXME: This is a basic implementation and the following should be fixed:
// - Fix TODOs if needed
// - Spawn threads to acquire the mutex
// - Maybe use tokio Mutex
async fn handle_socket<L: Llm, CR: ConversationRepository, R: Rag, S: Stt, T: Tts>(
  mut socket: WebSocket,
  app_state: LingooAppState<L, CR, R, S, T>,
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

      if let Ok(result) = result {
        result
      }

      // If we cannot send, it means the other part is disconnected
      bail!("Socket client side disconnected")
    };

  let mut session = VoiceChatSession::new(app_state);

  while let Some(raw_message) = socket.recv().await {
    // While there is a message, try to parse it into a [VoiceChatMessage] and handle it through
    // session. If message is invalid, reply with a message indicating it.
    if let Ok(raw_message) = raw_message {
      match raw_message {
        Message::Text(raw_text_message) => {
          if let Ok(parsed_message) = serde_json::from_str(&raw_text_message) {
            session.handle_message(parsed_message);
          } else {
            if let Err(_) = reply(&mut socket, &VoiceChatReplyMessage::Invalid).await {
              return;
            }
          }
        }
        _ => {
          if let Err(_) = reply(&mut socket, &VoiceChatReplyMessage::Invalid).await {
            return;
          }
        }
      };
    } else {
      return;
    }
  }
}
