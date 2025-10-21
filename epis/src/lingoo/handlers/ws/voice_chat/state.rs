use std::sync::mpsc::Receiver;

use epis_core::{
  non_empty_text::NonEmptyString, queue_worker_pool::queue_worker_pool::QueueWorkerPoolError,
};
use epis_stt::stt::SttError;

use crate::entities::common::Id;

pub enum InitVoiceChatState {
  Idle,
  ReceivingAudio(Vec<Result<Receiver<Result<NonEmptyString, SttError>>, QueueWorkerPoolError>>),
  GeneratingResponse(Vec<NonEmptyString>),
}

/// State of a voice chat session.
/// The state cycles this way:
/// Uninit -> Idle -> ReceivingAudio -> GeneratingResponse -> Idle -> ...
///
/// Meaning of each state:
/// - Uninit: No message is received yet, and the cid is unknown.
/// - Init
///   - Idle: The cid is received, but the actual audio is not received yet.
///   - ReceivingAudio: Audio is being received, queued, and passed to stt.
///   - GeneratingResponse: All audio chunks are received, and llm is generating response for the
///   text.
#[derive(Default)]
pub enum VoiceChatState {
  #[default]
  Uninit,
  Init(Id, InitVoiceChatState),
}
