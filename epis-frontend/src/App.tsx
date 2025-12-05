import { useState } from "react";
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
              <p>{(sessionClaims?.credit as string | undefined) ?? "?"} ⚡️</p>
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
