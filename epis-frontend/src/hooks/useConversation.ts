export type VoiceChatState = "idle" | "recording" | "waiting" | "responding";

export interface VoiceChatMessage {
  type: "VoiceChatInit";
  data: {
    cid: string;
  };
}

export interface VoiceChatPrompt {
  type: "VoiceChatPrompt";
  data: {
    audio_bytes_base64: string;
  };
}

export interface VoiceChatReplyMessage {
  type:
    | "Invalid"
    | "InvalidAudioBase64"
    | "InvalidSorroundAudio"
    | "InternalError"
    | "EmptyPrompt"
    | "LongPrompt"
    | "VoiceChatInitOk"
    | "VoiceChatAiReply"
    | "ZeroCharge";
  data?: {
    audio_bytes_base64?: string;
  };
}

// Custom hook for chatmate management
// This hook now simply returns the chatmate_id that is passed to it
// The chatmate_id should be set when a chatmate is selected or created
export function useChatmate(chatmateId: string | null) {
  return chatmateId;
}
