import { useState, useEffect, useRef } from "react";
import { useAuth } from "@clerk/clerk-react";
import { config } from "../config";

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

// Custom hook for conversation management
export function useConversation() {
  const [cid, setCid] = useState<string | null>(null);
  const conversationCreatedRef = useRef<boolean>(false);

  const { getToken } = useAuth();

  useEffect(() => {
    const createConversation = async () => {
      if (conversationCreatedRef.current) return;
      conversationCreatedRef.current = true;

      try {
	const token = await getToken();
        const response = await fetch(
          `${config.episServerUrl}/lingoo/conversation/create`,
          {
            method: "POST",
	    credentials: "include",
            headers: {
              "Content-Type": "application/json",
	      Authorization: `Bearer ${token}`,
            },
          }
        );

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        setCid(data.conversation_id);
        console.log("Conversation created:", data.conversation_id);
      } catch (error) {
        console.error("Failed to create conversation:", error);
      }
    };

    createConversation();
  }, [getToken]);

  return cid;
}



