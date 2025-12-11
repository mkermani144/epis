import { useState } from "react";
import { useAuth } from "@clerk/nextjs";
import ISO639 from "iso-639-1";
import { handshakeChatmate } from "../lib/api";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card";

const AVAILABLE_LANGUAGES = ["En", "Es", "Tr"] as const;

interface LanguageSelectionProps {
  existingLanguages: string[];
  onLanguageSelected: (chatmateId: string) => void;
  onCancel: () => void;
}

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
        <Card>
          <CardContent className="p-8 text-center">
             <p className="text-lg text-muted-foreground mb-4">
              All available languages have been added.
            </p>
            <Button onClick={onCancel} variant="outline">
              Go Back
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="w-full max-w-2xl mx-auto p-6">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl text-center">Select a Language</CardTitle>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 bg-destructive/10 border border-destructive/20 text-destructive rounded">
              {error}
            </div>
          )}
          <div className="grid grid-cols-1 gap-4 mb-6">
            {availableLanguages.map((language) => (
              <Button
                key={language}
                onClick={() => handleLanguageClick(language)}
                disabled={loading}
                variant="secondary"
                className="h-20 text-xl font-semibold hover:border-primary border-2 border-transparent transition-all"
              >
                {ISO639.getName(language.toLowerCase())}
              </Button>
            ))}
          </div>
        </CardContent>
        <CardFooter className="justify-center">
          <Button onClick={onCancel} variant="ghost" disabled={loading}>
            Cancel
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
