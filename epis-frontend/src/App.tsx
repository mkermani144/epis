import { useState } from "react";
import { VoiceCircle, StatusText } from "./components/VoiceCircle";
import { useConversation, useWebSocket, useAudioRecording } from "./hooks";
import {
  SignedIn,
  SignedOut,
  SignIn,
  SignInButton,
  SignUpButton,
  UserButton,
  useAuth,
} from "@clerk/clerk-react";
import { Button } from "./components/ui/button";

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
  const { sessionClaims } = useAuth();

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
    <div className="min-h-screen bg-gray-100 flex flex-col">
      <header className="absolute top-4 left-4 right-4 flex justify-between items-center z-10">
        <h1 className="text-4xl font-bold text-gray-800">Epis Lingoo</h1>
        <div className="flex items-center gap-2">
          <SignedOut>
            <SignInButton mode="modal">
              <Button variant="outline">Sign In</Button>
            </SignInButton>
            <SignUpButton mode="modal">
              <Button>Sign Up</Button>
            </SignUpButton>
          </SignedOut>
          <SignedIn>
	  <div className="flex items-center gap-2 border-2 border-solid border-slate-300 rounded cursor-default p-1 px-2">
	    <p>{sessionClaims?.charge ?? "?"} ⚡️</p>
            <UserButton />
	  </div>
          </SignedIn>
        </div>
      </header>

      <SignedIn>
        <div className="flex-1 flex flex-col items-center justify-center">
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
      </SignedIn>

      <SignedOut>
        <div className="flex-1 flex items-center justify-center">
          <SignIn />
        </div>
      </SignedOut>
    </div>
  );
}

export default App;
