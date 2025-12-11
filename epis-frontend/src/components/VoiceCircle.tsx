import type { VoiceChatState } from "../hooks/useConversation";
import { cn } from "@/lib/utils";

// Voice Circle Component
export function VoiceCircle({ state }: { state: VoiceChatState }) {
  const getCircleClasses = () => {
    const baseClasses = "rounded-full transition-all duration-300 ease-in-out shadow-lg";

    switch (state) {
      case "idle":
        return cn(baseClasses, "w-24 h-24 bg-muted hover:bg-muted/80 border-4 border-muted-foreground/20");
      case "recording":
        return cn(baseClasses, "w-32 h-32 bg-destructive animate-pulse shadow-destructive/50");
      case "waiting":
        return cn(baseClasses, "w-32 h-32 bg-muted-foreground animate-pulse");
      case "responding":
        return cn(baseClasses, "w-32 h-32 bg-primary animate-pulse shadow-primary/50");
      default:
        return cn(baseClasses, "w-24 h-24 bg-muted");
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
    <div className="flex flex-col items-center gap-2">
      <div className="text-lg font-medium text-foreground">
        {state === "idle" && "Tap or hold to record"}
        {state === "recording" && "Recording..."}
        {state === "waiting" && "Thinking..."}
        {state === "responding" && "Speaking..."}
      </div>

      <div className={cn("text-xs font-medium", isConnected ? "text-green-500" : "text-yellow-500")}>
        {isConnected ? "Connected" : "Connecting..."}
      </div>
    </div>
  );
}
