import { describe, expect, it } from "vitest";
import { action, sessionRequest, startSession, stopRequest, succeeded } from "./actions";

type Call = { path: string; init: RequestInit };

/** A fetch that records each call and answers with `status` and `body`. */
function server(status: number, body: unknown): { fetcher: typeof fetch; calls: Call[] } {
  const calls: Call[] = [];
  const fetcher = (async (path: string, init: RequestInit) => {
    calls.push({ path, init });
    return new Response(typeof body === "string" ? body : JSON.stringify(body), { status, headers: { "content-type": "application/json" } });
  }) as unknown as typeof fetch;
  return { fetcher, calls };
}

const identity = "thread:delegated-task:command%3Amcp%3Ad4040a34/child:1";

describe("the requests", () => {
  it("stops one mission, by ID, with the reason and no caller", () => {
    expect(stopRequest("atelier", "atelier-z4cr", "Not now.")).toEqual({ action: "stop", repo: "atelier", id: "atelier-z4cr", reason: "Not now." });
  });

  it("starts a planning session for a mission", () => {
    expect(sessionRequest("atelier", "atelier-z4cr")).toEqual({ repo: "atelier", mission: "atelier-z4cr" });
  });

  it("posts an action to /api/action as JSON with the app's header", async () => {
    const { fetcher, calls } = server(200, { result: "done", text: "Dropped.", paths: [], commit: "abc" });
    await action(stopRequest("atelier", "atelier-z4cr", "Not now."), fetcher);
    expect(calls).toHaveLength(1);
    expect(calls[0].path).toBe("/api/action");
    expect(calls[0].init.method).toBe("POST");
    expect(calls[0].init.headers).toMatchObject({ "x-atelier-desk": "1", "content-type": "application/json" });
    expect(JSON.parse(calls[0].init.body as string)).toEqual({ action: "stop", repo: "atelier", id: "atelier-z4cr", reason: "Not now." });
  });

  it("posts a session to /api/sessions with the app's header", async () => {
    const { fetcher, calls } = server(200, { result: "refused", text: "No project." });
    await startSession(sessionRequest("atelier", "atelier-z4cr"), fetcher);
    expect(calls[0].path).toBe("/api/sessions");
    expect(calls[0].init.headers).toMatchObject({ "x-atelier-desk": "1" });
    expect(JSON.parse(calls[0].init.body as string)).toEqual({ repo: "atelier", mission: "atelier-z4cr" });
  });
});

describe("each result's state", () => {
  it("keeps a done action as the server gave it, and counts it as succeeded", async () => {
    const done = { result: "done", text: "Dropped.\nPushed.", paths: [".atelier/missions/atelier-z4cr/mission.md"], commit: "abc" };
    const sent = await action(stopRequest("atelier", "atelier-z4cr", "Not now."), server(200, done).fetcher);
    expect(sent).toEqual({ state: "answered", response: done });
    expect(succeeded(sent)).toBe(true);
  });

  it("keeps a refusal's rules, text and next step as given, and does not count it as succeeded", async () => {
    const refused = { result: "refused", rules: [], text: "Mission atelier-z4cr is agreed.", next: "Run the commands its page gives." };
    const sent = await action(stopRequest("atelier", "atelier-z4cr", "Not now."), server(200, refused).fetcher);
    expect(sent).toEqual({ state: "answered", response: refused });
    expect(succeeded(sent)).toBe(false);
  });

  it("counts a started session as succeeded and a refused one not", async () => {
    const started = { result: "started", thread_id: identity, link: "https://app.t3.codes/threads/e/x", project_id: "p" };
    expect(succeeded(await startSession(sessionRequest("a", "m"), server(200, started).fetcher))).toBe(true);
    expect(succeeded(await startSession(sessionRequest("a", "m"), server(200, { result: "refused", text: "No." }).fetcher))).toBe(false);
  });

  it("reports a request the server turned away with its error, not as a result", async () => {
    const sent = await action(stopRequest("atelier", "atelier-z4cr", "Not now."), server(403, { error: "This action needs the header x-atelier-desk: 1." }).fetcher);
    expect(sent).toEqual({ state: "failed", error: "The Desk's server answered 403: This action needs the header x-atelier-desk: 1." });
    expect(succeeded(sent)).toBe(false);
  });

  it("reports a failure with no JSON, an answer it cannot read, and no connection", async () => {
    expect(await action(stopRequest("a", "m", "r"), server(500, "oops").fetcher)).toEqual({ state: "failed", error: "The Desk's server answered 500." });
    expect(await action(stopRequest("a", "m", "r"), server(200, { ok: true }).fetcher)).toEqual({ state: "failed", error: "The Desk's server sent an answer the page cannot read." });
    const offline = (async () => {
      throw new TypeError("Failed to fetch");
    }) as unknown as typeof fetch;
    expect(await action(stopRequest("a", "m", "r"), offline)).toEqual({ state: "failed", error: "Cannot reach the Desk's server: TypeError: Failed to fetch" });
  });
});
