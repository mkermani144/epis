use derive_more::Display;

use crate::{entities::common::Id, lingoo::handlers::ws::voice_chat::charge::Charge};

/// State of a voice chat session.
///
/// Meaning of each state:
/// - Uninit: No valid message is received yet, and the cid is unknown.
/// - Init: The cid is received and the service is ready to handle prompts
#[derive(Debug, Default, Display)]
pub enum VoiceChatState {
  #[default]
  #[display("Uninit")]
  Uninit,
  #[display("Init({cid}, {remaining_charge})")]
  Init { cid: Id, remaining_charge: Charge },
}
