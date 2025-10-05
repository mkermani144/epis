//! NOTE: This is a very basic implementation and will be changed soon

use axum::{
  extract::{
    State,
    ws::{Message, WebSocket, WebSocketUpgrade},
  },
  response::Response,
};
use epis_stt::stt::Stt;

use crate::{
  ai::llm::Llm, conversation::repository::ConversationRepository, entities::common::Id,
  http::server::LingooAppState, rag::rag::Rag,
};

pub async fn voice_chat<L: Llm, CR: ConversationRepository, R: Rag, S: Stt>(
  ws: WebSocketUpgrade,
  State(app_state): State<LingooAppState<L, CR, R, S>>,
) -> Response {
  ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

// FIXME: This is a basic implementation and the following should be fixed:
// 1. Message schemas
// 2. Invalid message handling
// 3. A possible better state machine implementation
// 4. Error handling (do not call unwrap)
// 5. Fix TODOs if needed
// 6. Spawn threads to acquire the mutex
// 7. Maybe use tokio Mutex
async fn handle_socket<L: Llm, CR: ConversationRepository, R: Rag, S: Stt>(
  mut socket: WebSocket,
  app_state: LingooAppState<L, CR, R, S>,
) {
  let mut cid: Option<Id> = None;
  while let Some(msg) = socket.recv().await {
    let msg = if let Ok(msg) = msg {
      match msg {
        Message::Text(text_msg) => {
          if let Ok(id) = Id::try_from(text_msg.to_string()) {
            cid = Some(id);
          }
          Message::Text("got cid".into())
        }
        Message::Binary(audio_msg) => {
          if cid.is_none() {
            Message::Text("cid is not received yet".into())
          } else {
            let user_message: String = app_state
              .stt
              .lock()
              .unwrap()
              // TODO: Do not call `to_vec`
              // TODO: Do not unwrap
              .speech_to_text(
                &audio_msg.to_vec().into(),
                epis_stt::models::SttLanguage::En,
              )
              .unwrap()
              // TODO: Do not collect - Call AI for each chunk instead
              .into_iter()
              .collect();
            println!("User message is: {}", user_message);

            let ai_reply = app_state
              .lingoo
              .chat(&cid.clone().unwrap(), user_message.try_into().unwrap())
              .await
              .unwrap();

            println!("Ai reply is: {}", ai_reply.as_ref());

            todo!("Implement Tts")
          }
        }
        _ => {
          println!("Some other type received");
          Message::Text("not-ok".into())
        }
      }
    } else {
      // client disconnected
      return;
    };

    if socket.send(msg).await.is_err() {
      // client disconnected
      return;
    }
  }
}
