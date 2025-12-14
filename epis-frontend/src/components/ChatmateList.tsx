import { useEffect, useState } from "react";
import { useAuth } from "@clerk/clerk-react";
import ISO639 from "iso-639-1";
import { fetchChatmates, type Chatmate } from "../lib/api";
import { Card, CardContent } from "./ui/card";
import { H3, Muted } from "./ui/typography";
import { Button } from "./ui/button";

interface ChatmateListProps {
  onSelectChatmate: (chatmateId: string) => void;
  onAddNew: () => void;
}

/**
 * Component that displays a list of chatmates in a grid layout
 */
export function ChatmateList({
  onSelectChatmate,
  onAddNew,
}: ChatmateListProps) {
  const [chatmates, setChatmates] = useState<Chatmate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { getToken } = useAuth();

  useEffect(() => {
    const loadChatmates = async () => {
      try {
        setLoading(true);
        setError(null);
        const token = await getToken();
        if (!token) {
          setError("Not authenticated");
          return;
        }

        const result = await fetchChatmates(token);
        if (!result.ok) {
          setError(result.error.message);
          return;
        }

        setChatmates(result.value.chatmates);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to load chatmates");
      } finally {
        setLoading(false);
      }
    };

    loadChatmates();
  }, [getToken]);

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Muted>Loading chatmates...</Muted>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Muted className="text-destructive">Error: {error}</Muted>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto p-6">
      <div className="grid grid-cols-2 gap-6">
        {chatmates.map((chatmate) => (
          <Button
            key={chatmate.chatmate_id}
            onClick={() => onSelectChatmate(chatmate.chatmate_id)}
            variant="outline"
            className="h-auto p-0 hover:bg-accent"
          >
            <Card className="w-full min-h-[200px] flex items-center justify-center cursor-pointer hover:shadow-lg transition-shadow">
              <CardContent className="text-center">
                <H3>{ISO639.getName(chatmate.language.toLowerCase())}</H3>
              </CardContent>
            </Card>
          </Button>
        ))}
        <Button
          onClick={onAddNew}
          variant="outline"
          className="h-auto p-0 hover:bg-accent"
        >
          <Card className="w-full min-h-[200px] flex items-center justify-center cursor-pointer border-dashed hover:shadow-lg transition-shadow">
            <CardContent className="text-center">
              <H3 className="text-muted-foreground">+ Add New</H3>
            </CardContent>
          </Card>
        </Button>
      </div>
    </div>
  );
}

