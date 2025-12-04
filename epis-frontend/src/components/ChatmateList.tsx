import { useEffect, useState } from "react";
import { useAuth } from "@clerk/clerk-react";
import { fetchChatmates, type Chatmate } from "../lib/api";
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
        <p className="text-gray-600">Loading chatmates...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <p className="text-red-600">Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto p-6">
      <div className="grid grid-cols-2 gap-6">
        {chatmates.map((chatmate) => (
          <button
            key={chatmate.chatmate_id}
            onClick={() => onSelectChatmate(chatmate.chatmate_id)}
            className="bg-white rounded-lg shadow-md p-8 hover:shadow-lg transition-shadow cursor-pointer border-2 border-transparent hover:border-blue-500 min-h-[200px] flex items-center justify-center"
          >
            <div className="text-center">
              <p className="text-2xl font-semibold text-gray-800">
                {chatmate.language}
              </p>
            </div>
          </button>
        ))}
        <button
          onClick={onAddNew}
          className="bg-gray-100 rounded-lg shadow-md p-8 hover:shadow-lg transition-shadow cursor-pointer border-2 border-dashed border-gray-300 hover:border-blue-500 min-h-[200px] flex items-center justify-center"
        >
          <div className="text-center">
            <p className="text-2xl font-semibold text-gray-600">+ Add New</p>
          </div>
        </button>
      </div>
    </div>
  );
}

