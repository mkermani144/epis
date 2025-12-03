//! Implement [AudioDuplex] for a thread-safe [WebSocket]

use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::Mutex;
use tracing::{instrument, trace, warn};

use crate::domain::{
  models::{EpisError, SimpleBytes},
  ports::AudioDuplex,
};

impl AudioDuplex for Arc<Mutex<WebSocket>> {
  #[instrument(skip_all)]
  async fn send(&mut self, audio_message: SimpleBytes) -> Result<(), EpisError> {
    self
      .lock()
      .await
      .send(Message::Binary(audio_message.into()))
      .await
      .inspect_err(|error| warn!(%error, "Failed to send audio message back to user"))
      .map_err(|_| EpisError::DuplexError)
  }

  #[instrument(skip_all)]
  async fn receive(&mut self) -> Result<SimpleBytes, EpisError> {
    if let Some(raw_message) = self.lock().await.recv().await {
      let raw_message = raw_message
        .inspect_err(|error| warn!(%error, "Failed to receive message"))
        .map_err(|_| EpisError::DuplexError)?;

      match raw_message {
        Message::Binary(bytes) => {
          trace!("Audio bytes received");
          return Ok(bytes.into());
        }
        _ => {
          trace!("An invalid non-audio message received");
          return Err(EpisError::DuplexError);
        }
      }
    }
    Err(EpisError::DuplexError)
  }
}
