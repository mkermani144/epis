import { useState, useRef } from "react";
import { VoiceCircle, StatusText } from "./components/VoiceCircle";
import { ChatmateList } from "./components/ChatmateList";
import { LanguageSelection } from "./components/LanguageSelection";
import { useChatmate, useWebSocket, useAudioRecording } from "./hooks";
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
import { H1, P } from "./components/ui/typography";
import { fetchChatmates } from "./lib/api";

type View = "list" | "language-selection" | "chat";

// Main App Component
function App() {
  const [view, setView] = useState<View>("list");
  const [selectedChatmateId, setSelectedChatmateId] = useState<string | null>(
    null
  );
  const [existingLanguages, setExistingLanguages] = useState<string[]>([]);
  const [state, setState] = useState<
    "idle" | "recording" | "waiting" | "responding"
  >("idle");
  const chatmateId = useChatmate(selectedChatmateId);
  const { isConnected, wsRef } = useWebSocket(
    view === "chat" ? chatmateId : null,
    setState
  );
  const { startRecording, stopRecording } = useAudioRecording(
    state,
    isConnected,
    wsRef,
    setState
  );
  const { sessionClaims, getToken } = useAuth();

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
    // If already recording, track press for potential stop on release
    if (state === "recording") {
      pressStartTimeRef.current = Date.now();
      return;
    }

    // If idle, start tracking press time
    if (state === "idle") {
      pressStartTimeRef.current = Date.now();
      recordingStartedByTimeoutRef.current = false;

      // Set up long press detection: if held > threshold, start recording
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

    // Cancel any pending long press timeout
    cancelLongPress();
    pressStartTimeRef.current = null;

    // If recording was started by timeout (long press), stop on release
    // Check both the ref and state to handle async state updates
    if (wasStartedByTimeout || currentState === "recording") {
      if (currentState === "recording") {
        stopRecording();
      } else if (wasStartedByTimeout) {
        // State hasn't updated yet, but recording was started
        // Use a small delay to check again
        setTimeout(() => {
          if (state === "recording") {
            stopRecording();
          }
        }, 10);
      }
      return;
    }

    // If idle, only start recording if it was a quick tap
    if (currentState === "idle" && pressDuration < TAP_THRESHOLD_MS) {
      // Quick tap: start recording and keep recording (toggle on)
      startRecording();
    }
  };

  const handleMouseDown = () => {
    handlePressStart();
  };

  const handleMouseUp = () => {
    handlePressEnd();
  };

  const handleTouchStart = (e: React.TouchEvent) => {
    // Prevent mouse events from firing on touch devices
    e.preventDefault();
    handlePressStart();
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    // Prevent mouse events from firing on touch devices
    e.preventDefault();
    handlePressEnd();
  };

  const handleTouchCancel = (e: React.TouchEvent) => {
    // Handle touch cancellation (e.g., scrolling)
    e.preventDefault();
    cancelLongPress();
    pressStartTimeRef.current = null;
    // If recording, stop it on cancel
    if (state === "recording") {
      stopRecording();
    }
  };

  const handleMouseLeave = () => {
    // If mouse leaves while pressing, treat as release
    handlePressEnd();
  };

  const handleSelectChatmate = (chatmateId: string) => {
    setSelectedChatmateId(chatmateId);
    setView("chat");
  };

  const handleAddNew = async () => {
    // Load existing languages
    try {
      const token = await getToken();
      if (token) {
        const result = await fetchChatmates(token);
        if (result.ok) {
          const languages = result.value.chatmates.map((c) => c.language);
          setExistingLanguages(languages);
        }
      }
    } catch (error) {
      console.error("Failed to load chatmates:", error);
    }
    setView("language-selection");
  };

  const handleLanguageSelected = (chatmateId: string) => {
    setSelectedChatmateId(chatmateId);
    setView("chat");
  };

  const handleBackToList = () => {
    setView("list");
    setSelectedChatmateId(null);
  };

  return (
    <div className="min-h-screen bg-background flex flex-col">
      <header className="absolute top-4 left-4 right-4 flex justify-between items-center z-10">
        <H1 className="text-foreground">Epis</H1>
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
            <div className="flex items-center gap-2 border-2 border-solid border-border rounded cursor-default p-1 px-2">
              <P className="m-0">{(sessionClaims?.credit as string | undefined) ?? "?"} ⚡️</P>
              <UserButton />
            </div>
          </SignedIn>
        </div>
      </header>

      <SignedIn>
        {view === "list" && (
          <div className="flex-1 flex flex-col items-center justify-center pt-20">
            <ChatmateList
              onSelectChatmate={handleSelectChatmate}
              onAddNew={handleAddNew}
            />
          </div>
        )}

        {view === "language-selection" && (
          <div className="flex-1 flex flex-col items-center justify-center pt-20">
            <LanguageSelection
              existingLanguages={existingLanguages}
              onLanguageSelected={handleLanguageSelected}
              onCancel={handleBackToList}
            />
          </div>
        )}

        {view === "chat" && (
          <div className="flex-1 flex flex-col items-center justify-center">
            <div className="absolute top-20 left-4">
              <Button onClick={handleBackToList} variant="outline">
                ← Back to Chatmates
              </Button>
            </div>
            {isConnected && (
              <div
                className="flex items-center justify-center"
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

            <StatusText state={state} isConnected={isConnected} />
          </div>
        )}
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
