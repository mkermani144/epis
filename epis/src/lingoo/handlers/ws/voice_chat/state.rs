use crate::entities::common::Id;

/// State of a voice chat session.
///
/// Meaning of each state:
/// - Uninit: No valid message is received yet, and the cid is unknown.
/// - Init: The cid is received and the service is ready to handle prompts
#[derive(Default)]
pub enum VoiceChatState {
  #[default]
  Uninit,
  Init(Id),
}
