use serde::{Deserialize, Serialize};

use crate::entities::common::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatMessage {
  /// Used to initialize a voice chat session by providing cid
  VoiceChatInit { cid: Id },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatReplyMessage {
  /// Used to indicate an invalid message is received on the server
  Invalid,
  /// Used to indicate voice chat initialization was successful
  VoiceChatInitOk,
}
