use serde::{Deserialize, Serialize};

use crate::entities::common::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatMessage {
  /// Used to initialize a voice chat session by providing cid
  VoiceChatInit {
    cid: Id,
  },
  VoiceChatPrompt {
    audio_bytes_base64: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatReplyMessage {
  /// Used to indicate an invalid message is received on the server
  Invalid,
  /// Used to indicate an invalid audio base 64
  InvalidAudioBase64,
  /// Used to indicate an invalid number of audio channels
  InvalidSorroundAudio,
  /// Used to indicate a server error, like 5xx
  InternalError,
  /// Used to indicate an empty prompt
  EmptyPrompt,
  /// Used to indicate voice chat initialization was successful
  VoiceChatInitOk,
  /// Used to send ai reply back to the user
  VoiceChatAiReply { audio_bytes_base64: String },
}
