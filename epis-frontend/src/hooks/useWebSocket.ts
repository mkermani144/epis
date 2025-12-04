import { useState, useEffect, useRef } from "react";
import { useAuth } from "@clerk/clerk-react";
import { config } from "../config";
import type { VoiceChatState } from "./useConversation";

// Audio playback utility
async function playAudio(
  arrayBuffer: ArrayBuffer,
  onStateChange: (state: VoiceChatState) => void
) {
  try {
    const blob = new Blob([arrayBuffer], {
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
  chatmateId: string | null,
  onStateChange: (state: VoiceChatState) => void
) {
  const [isConnected, setIsConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const { getToken } = useAuth();

  useEffect(() => {
    if (!chatmateId) return;

    const connectWebSocket = async () => {
      const token = await getToken();
      const ws = new WebSocket(
        `${config.episServerUrl.replace(
          "http",
          "ws"
        )}/v2/epis/ws/chat/${chatmateId}?jwt=${token}`
      );

      ws.onopen = () => {
        console.log("WebSocket connected");
        setIsConnected(true);
      };

      ws.onmessage = (event) => {
        try {
          playAudio(event.data, onStateChange);
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
  }, [chatmateId, onStateChange, getToken]);

  return { isConnected, wsRef };
}
