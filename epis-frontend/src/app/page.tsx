"use client";

import { ChatmateList } from "@/components/ChatmateList";
import { useRouter } from "next/navigation";
import { SignedIn, SignedOut, SignInButton, SignUpButton, UserButton, useAuth } from "@clerk/nextjs";
import { Button } from "@/components/ui/button";

export default function Home() {
  const router = useRouter();
  const { sessionClaims } = useAuth();

  return (
    <div className="min-h-screen bg-background flex flex-col">
      <header className="absolute top-4 left-4 right-4 flex justify-between items-center z-10 p-4">
        <div className="flex items-center gap-2">
           <div className="w-10 h-10 bg-primary text-primary-foreground rounded-full flex items-center justify-center font-bold text-2xl">E</div>
           <h1 className="text-2xl font-bold text-foreground hidden sm:block">Epis</h1>
        </div>
        <div className="flex items-center gap-4">
          <SignedOut>
            <SignInButton mode="modal">
              <Button variant="outline">Sign In</Button>
            </SignInButton>
            <SignUpButton mode="modal">
              <Button>Sign Up</Button>
            </SignUpButton>
          </SignedOut>
          <SignedIn>
            <div className="flex items-center gap-2 border border-border rounded-full px-4 py-1 bg-card">
              <p className="font-medium">{(sessionClaims?.credit as string | undefined) ?? "?"} ⚡️</p>
              <UserButton />
            </div>
          </SignedIn>
        </div>
      </header>

      <SignedIn>
        <div className="flex-1 flex flex-col items-center justify-center pt-24 pb-8">
           <h2 className="text-3xl font-bold mb-8">Your Chatmates</h2>
          <ChatmateList
            onSelectChatmate={(id) => router.push(`/chat/${id}`)}
            onAddNew={() => router.push("/new")}
          />
        </div>
      </SignedIn>

      <SignedOut>
        <div className="flex-1 flex flex-col items-center justify-center p-8 text-center">
          <div className="w-24 h-24 bg-primary text-primary-foreground rounded-full flex items-center justify-center font-bold text-6xl mb-8">E</div>
          <h1 className="text-4xl font-bold mb-4">Welcome to Epis</h1>
          <p className="text-xl text-muted-foreground mb-8 max-w-md">
            Your AI language learning companion. Practice speaking naturally with AI.
          </p>
           <SignInButton mode="modal">
              <Button size="lg" className="rounded-full px-8">Get Started</Button>
            </SignInButton>
        </div>
      </SignedOut>
    </div>
  );
}
