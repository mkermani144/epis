import { useState } from "react";
import { VoiceCircle, StatusText } from "./components/VoiceCircle";
import { useConversation, useWebSocket, useAudioRecording } from "./hooks";

// Main App Component
function App() {
  const [state, setState] = useState<
    "idle" | "recording" | "waiting" | "responding"
  >("idle");
  const cid = useConversation();
  const { isConnected, wsRef } = useWebSocket(cid, setState);
  const { startRecording, stopRecording } = useAudioRecording(
    state,
    isConnected,
    wsRef,
    setState
  );

  const handleMouseDown = () => {
    if (state === "idle") {
      startRecording();
    }
  };

  const handleMouseUp = () => {
    if (state === "recording") {
      stopRecording();
    }
  };

  const handleTouchStart = () => {
    if (state === "idle") {
      startRecording();
    }
  };

  const handleTouchEnd = () => {
    if (state === "recording") {
      stopRecording();
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
      <h1 className="text-4xl font-bold text-gray-800 mb-12">Epis Lingoo</h1>

      <div
        className="flex items-center justify-center"
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
        onTouchStart={handleTouchStart}
        onTouchEnd={handleTouchEnd}
      >
        <VoiceCircle state={state} />
      </div>

      <StatusText state={state} isConnected={isConnected} />
    </div>
  );
}

export default App;
