"use client";

import { VoiceChatSession } from "@/components/VoiceChatSession";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";

export default function ChatPage() {
  const params = useParams();
  const [chatmateId, setChatmateId] = useState<string | null>(null);

  useEffect(() => {
    if (params?.chatmateId) {
      setChatmateId(params.chatmateId as string);
    }
  }, [params]);

  if (!chatmateId) return null;

  return (
    <div className="min-h-screen bg-background flex flex-col">
       <VoiceChatSession chatmateId={chatmateId} />
    </div>
  );
}
