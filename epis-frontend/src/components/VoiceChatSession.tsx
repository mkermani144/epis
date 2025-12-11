"use client";

import { useState, useRef } from "react";
import { VoiceCircle, StatusText } from "@/components/VoiceCircle";
import { useWebSocket, useAudioRecording } from "@/hooks";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { ArrowLeft } from "lucide-react";

interface VoiceChatSessionProps {
  chatmateId: string;
}

export function VoiceChatSession({ chatmateId }: VoiceChatSessionProps) {
  const router = useRouter();
  const [state, setState] = useState<
    "idle" | "recording" | "waiting" | "responding"
  >("idle");

  const { isConnected, wsRef } = useWebSocket(chatmateId, setState);
  const { startRecording, stopRecording } = useAudioRecording(
    state,
    isConnected,
    wsRef,
    setState
  );

  // Threshold to distinguish quick tap from long press (1000ms)
  const TAP_THRESHOLD_MS = 1000;
  const pressStartTimeRef = useRef<number | null>(null);
  const longPressTimeoutRef = useRef<number | null>(null);
  const recordingStartedByTimeoutRef = useRef<boolean>(false);

  const cancelLongPress = () => {
    if (longPressTimeoutRef.current !== null) {
      clearTimeout(longPressTimeoutRef.current);
      longPressTimeoutRef.current = null;
    }
    recordingStartedByTimeoutRef.current = false;
  };

  const handlePressStart = () => {
    if (state === "recording") {
      pressStartTimeRef.current = Date.now();
      return;
    }

    if (state === "idle") {
      pressStartTimeRef.current = Date.now();
      recordingStartedByTimeoutRef.current = false;

      longPressTimeoutRef.current = window.setTimeout(() => {
        if (pressStartTimeRef.current !== null && state === "idle") {
          recordingStartedByTimeoutRef.current = true;
          startRecording();
        }
        longPressTimeoutRef.current = null;
      }, TAP_THRESHOLD_MS);
    }
  };

  const handlePressEnd = () => {
    if (pressStartTimeRef.current === null) {
      return;
    }

    const pressDuration = Date.now() - pressStartTimeRef.current;
    const currentState = state;
    const wasStartedByTimeout = recordingStartedByTimeoutRef.current;

    cancelLongPress();
    pressStartTimeRef.current = null;

    if (wasStartedByTimeout || currentState === "recording") {
      if (currentState === "recording") {
        stopRecording();
      } else if (wasStartedByTimeout) {
        // State hasn't updated yet, but recording was started
        setTimeout(() => {
            stopRecording();
        }, 10);
      }
      return;
    }

    if (currentState === "idle" && pressDuration < TAP_THRESHOLD_MS) {
      startRecording();
    }
  };

  const handleMouseDown = () => handlePressStart();
  const handleMouseUp = () => handlePressEnd();
  const handleMouseLeave = () => handlePressEnd();
  
  const handleTouchStart = (e: React.TouchEvent) => {
    e.preventDefault();
    handlePressStart();
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    e.preventDefault();
    handlePressEnd();
  };

  const handleTouchCancel = (e: React.TouchEvent) => {
    e.preventDefault();
    cancelLongPress();
    pressStartTimeRef.current = null;
    if (state === "recording") {
      stopRecording();
    }
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center relative min-h-[calc(100vh-80px)]">
      <div className="absolute top-4 left-4">
        <Button onClick={() => router.push("/")} variant="outline" className="gap-2">
          <ArrowLeft size={16} />
          Back to Chatmates
        </Button>
      </div>
      
      {isConnected && (
        <div
          className="flex items-center justify-center cursor-pointer active:scale-95 transition-transform"
          onMouseDown={handleMouseDown}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseLeave}
          onTouchStart={handleTouchStart}
          onTouchEnd={handleTouchEnd}
          onTouchCancel={handleTouchCancel}
        >
          <VoiceCircle state={state} />
        </div>
      )}

      <div className="mt-8">
         <StatusText state={state} isConnected={isConnected} />
      </div>
    </div>
  );
}
