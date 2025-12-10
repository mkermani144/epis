import type { VoiceChatState } from "../hooks/useConversation";

// Voice Circle Component
export function VoiceCircle({ state }: { state: VoiceChatState }) {
  const getCircleClasses = () => {
    const baseClasses = "rounded-full transition-all duration-300 ease-in-out";

    switch (state) {
      case "idle":
        return `${baseClasses} w-24 h-24 bg-gray-400 hover:bg-gray-500 cursor-pointer`;
      case "recording":
        return `${baseClasses} w-32 h-32 bg-red-500 animate-pulse`;
      case "waiting":
        return `${baseClasses} w-32 h-32 bg-gray-300 animate-pulse`;
      case "responding":
        return `${baseClasses} w-32 h-32 bg-blue-500 animate-pulse`;
      default:
        return `${baseClasses} w-24 h-24 bg-gray-400`;
    }
  };

  return <div className={getCircleClasses()} />;
}

// Status Text Component
export function StatusText({
  state,
  isConnected,
}: {
  state: VoiceChatState;
  isConnected: boolean;
}) {
  return (
    <>
      <div className="mt-8 text-sm text-gray-600">
        {state === "idle" && "Tap or hold to record"}
        {state === "recording" && "Recording... Release to send"}
        {state === "waiting" && "Processing..."}
        {state === "responding" && "Playing response..."}
      </div>

      <div className="mt-4 text-xs text-gray-500">
        {isConnected ? "Connected" : "Connecting..."}
      </div>
    </>
  );
}
