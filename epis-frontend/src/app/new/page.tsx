"use client";

import { useEffect, useState } from "react";
import { LanguageSelection } from "@/components/LanguageSelection";
import { useRouter } from "next/navigation";
import { useAuth } from "@clerk/nextjs";
import { fetchChatmates } from "@/lib/api";

export default function NewChatmatePage() {
  const router = useRouter();
  const { getToken } = useAuth();
  const [existingLanguages, setExistingLanguages] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadLanguages = async () => {
      try {
        const token = await getToken();
        if (token) {
          const result = await fetchChatmates(token);
          if (result.ok) {
             setExistingLanguages(result.value.chatmates.map(c => c.language));
          }
        }
      } catch (error) {
        console.error("Failed to load chatmates", error);
      } finally {
        setLoading(false);
      }
    };
    loadLanguages();
  }, [getToken]);

  if (loading) {
      return (
          <div className="min-h-screen flex items-center justify-center">
              <p className="text-muted-foreground">Loading...</p>
          </div>
      )
  }

  return (
    <div className="min-h-screen bg-background flex flex-col items-center justify-center pt-20">
      <LanguageSelection
        existingLanguages={existingLanguages}
        onLanguageSelected={(id) => router.push(`/chat/${id}`)}
        onCancel={() => router.push("/")}
      />
    </div>
  );
}
