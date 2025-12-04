import { config } from "../config";

export interface Chatmate {
  chatmate_id: string;
  language: string;
}

export interface ListChatmatesResponse {
  chatmates: Chatmate[];
}

export interface HandshakeChatmateRequest {
  language: string;
}

export interface HandshakeChatmateResponse {
  chatmate_id: string;
}

/**
 * Fetch all chatmates for the current user
 */
export async function fetchChatmates(
  token: string
): Promise<Result<ListChatmatesResponse, Error>> {
  try {
    const response = await fetch(`${config.episServerUrl}/v2/epis/chatmate`, {
      method: "GET",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    return { ok: true, value: data };
  } catch (error) {
    return {
      ok: false,
      error: error instanceof Error ? error : new Error(String(error)),
    };
  }
}

/**
 * Handshake (create) a new chatmate for the specified language
 */
export async function handshakeChatmate(
  token: string,
  request: HandshakeChatmateRequest
): Promise<Result<HandshakeChatmateResponse, Error>> {
  try {
    const response = await fetch(
      `${config.episServerUrl}/v2/epis/chatmate/handshake`,
      {
        method: "POST",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(request),
      }
    );

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(
        `HTTP error! status: ${response.status}, message: ${errorText}`
      );
    }

    const data = await response.json();
    return { ok: true, value: data };
  } catch (error) {
    return {
      ok: false,
      error: error instanceof Error ? error : new Error(String(error)),
    };
  }
}

type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

