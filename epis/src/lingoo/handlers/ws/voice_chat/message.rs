use derive_more::{Debug, Display};
use serde::{Deserialize, Serialize};

use crate::entities::common::Id;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatMessage {
  /// Used to initialize a voice chat session by providing cid
  #[display("VoiceChatInit(cid={cid})")]
  VoiceChatInit { cid: Id },
  #[display("VoiceChatPrompt(<base64>)")]
  VoiceChatPrompt {
    #[debug(skip)]
    audio_bytes_base64: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type", content = "data")]
pub enum VoiceChatReplyMessage {
  /// Used to indicate an invalid message is received on the server
  #[display("Invalid")]
  Invalid,
  /// Used to indicate an invalid audio base 64
  #[display("InvalidAudioBase64")]
  InvalidAudioBase64,
  /// Used to indicate zero remaining charge
  #[display("ZeroCharge")]
  ZeroCharge,
  /// Used to indicate an invalid number of audio channels
  #[display("InvalidSorroundAudio")]
  InvalidSorroundAudio,
  /// Used to indicate a server error, like 5xx
  #[display("InternalError")]
  InternalError,
  /// Used to indicate an empty prompt
  #[display("EmptyPrompt")]
  EmptyPrompt,
  /// Used to indicate voice chat initialization was successful
  #[display("VoiceChatInitOk")]
  VoiceChatInitOk,
  /// Used to send ai reply back to the user
  #[display("VoiceChatAiReply(<base64>)")]
  VoiceChatAiReply {
    #[debug(skip)]
    audio_bytes_base64: String,
  },
}
