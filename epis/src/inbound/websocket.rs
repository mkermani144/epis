//! Implement [AudioDuplex] for a thread-safe [WebSocket]

use std::sync::{Arc, Mutex};

use axum::extract::ws::WebSocket;

use crate::domain::{
  models::{EpisAudioMessage, EpisError},
  ports::AudioDuplex,
};

impl AudioDuplex for Arc<Mutex<WebSocket>> {
  async fn send(&mut self, audio_message: EpisAudioMessage) -> Result<(), EpisError> {
    todo!()
  }
  async fn receive(&mut self) -> Result<EpisAudioMessage, EpisError> {
    todo!()
  }
}
