import type { ActRequest } from "../api/ActRequest";
import type { ActResponse } from "../api/ActResponse";
import type { ApiError } from "../api/ApiError";
import type { SessionRequest } from "../api/SessionRequest";
import type { SessionResponse } from "../api/SessionResponse";

/** The header the server requires on an act or a session; a page of another site cannot send it. */
export const APP_HEADER = "x-atelier-desk";

export function stopRequest(repo: string, mission: string, reason: string): ActRequest {
  return { act: "stop", repo, id: mission, reason };
}

export function sessionRequest(repo: string, mission: string): SessionRequest {
  return { repo, mission };
}

/**
 * Where a control for an act or a session stands. `failed` is a request the
 * server did not answer with a result, such as a 403 or no connection;
 * `answered` holds the result it gave, done or refused.
 */
export type Sent<R> = { state: "idle" } | { state: "confirming" } | { state: "sending" } | { state: "answered"; response: R } | { state: "failed"; error: string };

/** Sends `body` to `path` as the app, and gives back the server's result or why there is none. */
export async function send<R>(path: "/api/act" | "/api/sessions", body: ActRequest | SessionRequest, fetcher: typeof fetch = fetch): Promise<Sent<R>> {
  let response: Response;
  try {
    response = await fetcher(path, {
      method: "POST",
      headers: { "content-type": "application/json", accept: "application/json", [APP_HEADER]: "1" },
      body: JSON.stringify(body),
    });
  } catch (error) {
    return { state: "failed", error: `Cannot reach the Desk's server: ${String(error)}` };
  }
  const parsed: unknown = await response.json().catch(() => null);
  if (!response.ok) {
    const error = (parsed as ApiError | null)?.error;
    return { state: "failed", error: error ? `The Desk's server answered ${response.status}: ${error}` : `The Desk's server answered ${response.status}.` };
  }
  const result = (parsed as { result?: unknown } | null)?.result;
  if (typeof result !== "string") return { state: "failed", error: "The Desk's server sent an answer the page cannot read." };
  return { state: "answered", response: parsed as R };
}

export const act = (body: ActRequest, fetcher?: typeof fetch) => send<ActResponse>("/api/act", body, fetcher);
export const startSession = (body: SessionRequest, fetcher?: typeof fetch) => send<SessionResponse>("/api/sessions", body, fetcher);

/** Whether a sent control has finished with nothing left to do: the act was done or the session started. */
export function succeeded(sent: Sent<ActResponse> | Sent<SessionResponse>): boolean {
  return sent.state === "answered" && (sent.response.result === "done" || sent.response.result === "started");
}
