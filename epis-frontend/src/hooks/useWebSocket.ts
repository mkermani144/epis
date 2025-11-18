import { useState, useEffect, useRef } from "react";
import { useAuth } from "@clerk/clerk-react"
import { config } from "../config";
import type {
  VoiceChatState,
  VoiceChatMessage,
  VoiceChatReplyMessage,
} from "./useConversation";

// Audio playback utility
async function playAudio(
  base64Audio: string,
  onStateChange: (state: VoiceChatState) => void
) {
  try {
    const audioData = atob(base64Audio);
    const audioBuffer = new Uint8Array(audioData.length);
    for (let i = 0; i < audioData.length; i++) {
      audioBuffer[i] = audioData.charCodeAt(i);
    }

    const blob = new Blob([audioBuffer], {
      type: "audio/wav; codecs=1",
    });
    const audioUrl = URL.createObjectURL(blob);
    const audio = new Audio(audioUrl);

    audio.preload = "auto";

    audio.onended = () => {
      URL.revokeObjectURL(audioUrl);
      onStateChange("idle");
    };

    audio.onerror = (error) => {
      console.error("Audio playback error:", error);
      console.error("Audio format: 32-bit float WAV, 24kHz, mono");
      URL.revokeObjectURL(audioUrl);
      onStateChange("idle");
    };

    await audio.play();
    console.log("Playing AI response audio (32-bit float WAV, 24kHz)");
  } catch (error) {
    console.error("Failed to play audio:", error);
    console.error("Expected format: 32-bit float WAV, 24kHz sample rate, mono");
    onStateChange("idle");
  }
}

// Custom hook for WebSocket connection
export function useWebSocket(
  cid: string | null,
  onStateChange: (state: VoiceChatState) => void
) {
  const [isConnected, setIsConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const { getToken } = useAuth()

  useEffect(() => {
    if (!cid) return;

    const connectWebSocket = async () => {
      const token = await getToken();
      const ws = new WebSocket(
        `${config.episServerUrl.replace("http", "ws")}/ws/lingoo/voice-chat?token=${token}`
      );

      ws.onopen = () => {
        console.log("WebSocket connected");
        setIsConnected(true);

        const initMessage: VoiceChatMessage = {
          type: "VoiceChatInit",
          data: { cid },
        };
        ws.send(JSON.stringify(initMessage));
      };

      ws.onmessage = (event) => {
        try {
          const message: VoiceChatReplyMessage = JSON.parse(event.data);
          console.log("Received message:", message);

          switch (message.type) {
            case "VoiceChatInitOk":
              console.log("Voice chat initialized successfully");
              break;
            case "VoiceChatAiReply":
              if (message.data?.audio_bytes_base64) {
                playAudio(message.data.audio_bytes_base64, onStateChange);
                onStateChange("responding");
              }
              break;
            case "Invalid":
            case "InvalidAudioBase64":
            case "InvalidSorroundAudio":
            case "InternalError":
            case "EmptyPrompt":
            case "ZeroCharge":
            case "LongPrompt":
              console.error("Server error:", message.type);
              onStateChange("idle");
              break;
          }
        } catch (error) {
          console.error("Failed to parse websocket message:", error);
        }
      };

      ws.onclose = () => {
        console.log("WebSocket disconnected");
        setIsConnected(false);
        onStateChange("idle");
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        setIsConnected(false);
        onStateChange("idle");
      };

      wsRef.current = ws;
    };

    connectWebSocket();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [cid, onStateChange]);

  return { isConnected, wsRef };
}
