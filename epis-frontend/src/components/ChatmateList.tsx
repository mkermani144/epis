import { useEffect, useState } from "react";
import { useAuth } from "@clerk/nextjs";
import ISO639 from "iso-639-1";
import { fetchChatmates, type Chatmate } from "../lib/api";
import { Card, CardContent } from "@/components/ui/card";
import { Plus } from "lucide-react";

interface ChatmateListProps {
  onSelectChatmate: (chatmateId: string) => void;
  onAddNew: () => void;
}

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
        <p className="text-muted-foreground">Loading chatmates...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <p className="text-destructive">Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto p-6">
      <div className="grid grid-cols-2 md:grid-cols-3 gap-6">
        {chatmates.map((chatmate) => (
          <Card
            key={chatmate.chatmate_id}
            className="cursor-pointer hover:shadow-lg transition-all hover:border-primary/50 group"
            onClick={() => onSelectChatmate(chatmate.chatmate_id)}
          >
            <CardContent className="flex flex-col items-center justify-center min-h-[200px] p-6">
              <div className="text-4xl mb-4 font-bold text-primary">
                 {ISO639.getName(chatmate.language.toLowerCase())}
              </div>
              <p className="text-muted-foreground group-hover:text-primary transition-colors">
                Chat now
              </p>
            </CardContent>
          </Card>
        ))}
        <Card
          className="cursor-pointer hover:shadow-lg transition-all border-dashed border-2 hover:border-primary/50 bg-muted/50 hover:bg-muted"
          onClick={onAddNew}
        >
          <CardContent className="flex flex-col items-center justify-center min-h-[200px] p-6">
             <Plus className="w-12 h-12 text-muted-foreground mb-4" />
             <p className="text-xl font-semibold text-muted-foreground">Add New</p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
