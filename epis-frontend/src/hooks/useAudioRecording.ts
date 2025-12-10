import { useRef } from "react";
import type { VoiceChatState } from "./useConversation";

// Audio conversion utilities
async function convertToWav(webmBlob: Blob): Promise<Blob> {
  return new Promise((resolve, reject) => {
    // Validate blob before processing
    if (!webmBlob || webmBlob.size === 0) {
      reject(new Error("Invalid or empty audio blob"));
      return;
    }

    const audioContext = new (window.AudioContext ||
      (window as unknown as { webkitAudioContext: typeof AudioContext })
        .webkitAudioContext)();
    const fileReader = new FileReader();

    fileReader.onload = async () => {
      try {
        const arrayBuffer = fileReader.result as ArrayBuffer;

        if (!arrayBuffer || arrayBuffer.byteLength === 0) {
          reject(new Error("Empty array buffer"));
          return;
        }

        const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

        if (audioBuffer.length === 0) {
          reject(new Error("Decoded audio buffer is empty"));
          return;
        }

        const wavBuffer = audioBufferToWav(audioBuffer);
        const wavBlob = new Blob([wavBuffer], { type: "audio/wav" });
        resolve(wavBlob);
      } catch (error) {
        reject(
          error instanceof Error
            ? error
            : new Error(`Failed to decode audio: ${String(error)}`)
        );
      }
    };

    fileReader.onerror = () => {
      reject(new Error("Failed to read audio blob"));
    };

    fileReader.readAsArrayBuffer(webmBlob);
  });
}

function audioBufferToWav(audioBuffer: AudioBuffer): ArrayBuffer {
  const numberOfChannels = audioBuffer.numberOfChannels;
  const sampleRate = audioBuffer.sampleRate;
  const length = audioBuffer.length;

  const buffer = new ArrayBuffer(44 + length * numberOfChannels * 2);
  const view = new DataView(buffer);

  const writeString = (offset: number, string: string) => {
    for (let i = 0; i < string.length; i++) {
      view.setUint8(offset + i, string.charCodeAt(i));
    }
  };

  writeString(0, "RIFF");
  view.setUint32(4, 36 + length * numberOfChannels * 2, true);
  writeString(8, "WAVE");
  writeString(12, "fmt ");
  view.setUint32(16, 16, true);
  view.setUint16(20, 1, true);
  view.setUint16(22, numberOfChannels, true);
  view.setUint32(24, sampleRate, true);
  view.setUint32(28, sampleRate * numberOfChannels * 2, true);
  view.setUint16(32, numberOfChannels * 2, true);
  view.setUint16(34, 16, true);
  writeString(36, "data");
  view.setUint32(40, length * numberOfChannels * 2, true);

  let offset = 44;
  for (let i = 0; i < length; i++) {
    for (let channel = 0; channel < numberOfChannels; channel++) {
      const sample = Math.max(
        -1,
        Math.min(1, audioBuffer.getChannelData(channel)[i])
      );
      view.setInt16(
        offset,
        sample < 0 ? sample * 0x8000 : sample * 0x7fff,
        true
      );
      offset += 2;
    }
  }

  return buffer;
}

// Custom hook for audio recording
export function useAudioRecording(
  state: VoiceChatState,
  isConnected: boolean,
  wsRef: React.RefObject<WebSocket | null>,
  onStateChange: (state: VoiceChatState) => void
) {
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const recordingStartTimeRef = useRef<number | null>(null);
  const streamRef = useRef<MediaStream | null>(null);

  // Minimum recording duration in milliseconds (100ms)
  const MIN_RECORDING_DURATION_MS = 100;
  // Minimum blob size in bytes to ensure valid audio data
  const MIN_BLOB_SIZE_BYTES = 100;

  const startRecording = async () => {
    if (state !== "idle" || !isConnected) return;

    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      streamRef.current = stream;
      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: "audio/webm;codecs=opus",
      });

      const chunks: Blob[] = [];

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          chunks.push(event.data);
        }
      };

      mediaRecorder.onstop = async () => {
        const recordingDuration = recordingStartTimeRef.current
          ? Date.now() - recordingStartTimeRef.current
          : 0;

        // Reset recording start time
        recordingStartTimeRef.current = null;

        // Clean up stream tracks
        if (streamRef.current) {
          streamRef.current.getTracks().forEach((track) => track.stop());
          streamRef.current = null;
        }

        const blob = new Blob(chunks, { type: "audio/webm" });

        // Validate recording duration and blob size
        if (
          recordingDuration < MIN_RECORDING_DURATION_MS ||
          blob.size < MIN_BLOB_SIZE_BYTES
        ) {
          console.warn(
            "Recording too short or invalid, ignoring:",
            `duration: ${recordingDuration}ms, size: ${blob.size} bytes`
          );
          onStateChange("idle");
          return;
        }

        try {
          const wavBlob = await convertToWav(blob);

          if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
            wsRef.current.send(await wavBlob.arrayBuffer());
            onStateChange("waiting");
          } else {
            onStateChange("idle");
          }
        } catch (error) {
          console.error("Failed to convert audio:", error);
          onStateChange("idle");
        }
      };

      recordingStartTimeRef.current = Date.now();
      mediaRecorder.start();
      mediaRecorderRef.current = mediaRecorder;
      onStateChange("recording");
    } catch (error) {
      console.error("Failed to start recording:", error);
      recordingStartTimeRef.current = null;
      if (streamRef.current) {
        streamRef.current.getTracks().forEach((track) => track.stop());
        streamRef.current = null;
      }
      onStateChange("idle");
    }
  };

  const stopRecording = () => {
    if (mediaRecorderRef.current && state === "recording") {
      mediaRecorderRef.current.stop();
      mediaRecorderRef.current = null;
    }
  };

  return { startRecording, stopRecording };
}
