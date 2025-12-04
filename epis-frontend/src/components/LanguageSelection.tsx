import { useState } from "react";
import { useAuth } from "@clerk/clerk-react";
import { handshakeChatmate } from "../lib/api";
import { Button } from "./ui/button";

const AVAILABLE_LANGUAGES = ["En", "Es", "Tr"] as const;

interface LanguageSelectionProps {
  existingLanguages: string[];
  onLanguageSelected: (chatmateId: string) => void;
  onCancel: () => void;
}

/**
 * Component that displays available languages for creating a new chatmate
 */
export function LanguageSelection({
  existingLanguages,
  onLanguageSelected,
  onCancel,
}: LanguageSelectionProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { getToken } = useAuth();

  const availableLanguages = AVAILABLE_LANGUAGES.filter(
    (lang) => !existingLanguages.includes(lang)
  );

  const handleLanguageClick = async (language: string) => {
    try {
      setLoading(true);
      setError(null);
      const token = await getToken();
      if (!token) {
        setError("Not authenticated");
        return;
      }

      const result = await handshakeChatmate(token, { language });
      if (!result.ok) {
        setError(result.error.message);
        return;
      }

      onLanguageSelected(result.value.chatmate_id);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create chatmate");
    } finally {
      setLoading(false);
    }
  };

  if (availableLanguages.length === 0) {
    return (
      <div className="w-full max-w-2xl mx-auto p-6">
        <div className="bg-white rounded-lg shadow-md p-8 text-center">
          <p className="text-lg text-gray-600 mb-4">
            All available languages have been added.
          </p>
          <Button onClick={onCancel} variant="outline">
            Go Back
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full max-w-2xl mx-auto p-6">
      <div className="bg-white rounded-lg shadow-md p-8">
        <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center">
          Select a Language
        </h2>
        {error && (
          <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
            {error}
          </div>
        )}
        <div className="grid grid-cols-1 gap-4 mb-6">
          {availableLanguages.map((language) => (
            <button
              key={language}
              onClick={() => handleLanguageClick(language)}
              disabled={loading}
              className="bg-gray-100 rounded-lg p-6 hover:bg-gray-200 transition-colors cursor-pointer border-2 border-transparent hover:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <p className="text-xl font-semibold text-gray-800">{language}</p>
            </button>
          ))}
        </div>
        <div className="flex justify-center">
          <Button onClick={onCancel} variant="outline" disabled={loading}>
            Cancel
          </Button>
        </div>
      </div>
    </div>
  );
}

