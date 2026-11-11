import { describe, expect, it } from "vitest";
import type { Question } from "../api/Question";
import { act, decideRequest, readyRequest, sessionRequest, startSession, succeeded, withPending } from "./acts";

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
  it("says go on one mission, by ID, with no caller", () => {
    expect(readyRequest("atelier", "atelier-z4cr")).toEqual({ act: "ready", repo: "atelier", ids: ["atelier-z4cr"] });
  });

  it("answers an item's question, with or without a settlement", () => {
    expect(decideRequest("atelier", "atelier-uwxy", "Yes.", null)).toEqual({ act: "decide", repo: "atelier", id: "atelier-uwxy", answer: "Yes.", settle: null });
    expect(decideRequest("atelier", "atelier-uwxy", "Later.", "defer")).toEqual({ act: "decide", repo: "atelier", id: "atelier-uwxy", answer: "Later.", settle: "defer" });
  });

  it("starts a planning session for a mission", () => {
    expect(sessionRequest("atelier", "atelier-z4cr")).toEqual({ repo: "atelier", mission: "atelier-z4cr" });
  });

  it("posts an act to /api/act as JSON with the app's header", async () => {
    const { fetcher, calls } = server(200, { result: "done", text: "Made ready.", paths: [], commit: "abc" });
    await act(readyRequest("atelier", "atelier-z4cr"), fetcher);
    expect(calls).toHaveLength(1);
    expect(calls[0].path).toBe("/api/act");
    expect(calls[0].init.method).toBe("POST");
    expect(calls[0].init.headers).toMatchObject({ "x-atelier-desk": "1", "content-type": "application/json" });
    expect(JSON.parse(calls[0].init.body as string)).toEqual({ act: "ready", repo: "atelier", ids: ["atelier-z4cr"] });
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
  it("keeps a done act as the server gave it, and counts it as succeeded", async () => {
    const done = { result: "done", text: "Answered.\nNext: submit.", paths: [".atelier/items/atelier-uwxy/item.md"], commit: null };
    const sent = await act(decideRequest("atelier", "atelier-uwxy", "Yes.", null), server(200, done).fetcher);
    expect(sent).toEqual({ state: "answered", response: done });
    expect(succeeded(sent)).toBe(true);
  });

  it("keeps a refusal's rules, text and next step as given, and does not count it as succeeded", async () => {
    const refused = { result: "refused", rules: ["ready.plan_reviewed"], text: "`atelier ready` refused; the plan has no review.", next: "Ask for a plan review." };
    const sent = await act(readyRequest("atelier", "atelier-z4cr"), server(200, refused).fetcher);
    expect(sent).toEqual({ state: "answered", response: refused });
    expect(succeeded(sent)).toBe(false);
  });

  it("counts a started session as succeeded and a refused one not", async () => {
    const started = { result: "started", thread_id: identity, link: "https://app.t3.codes/threads/e/x", project_id: "p" };
    expect(succeeded(await startSession(sessionRequest("a", "m"), server(200, started).fetcher))).toBe(true);
    expect(succeeded(await startSession(sessionRequest("a", "m"), server(200, { result: "refused", text: "No." }).fetcher))).toBe(false);
  });

  it("reports a request the server turned away with its error, not as a result", async () => {
    const sent = await act(readyRequest("atelier", "atelier-z4cr"), server(403, { error: "This act needs the header x-atelier-desk: 1." }).fetcher);
    expect(sent).toEqual({ state: "failed", error: "The Desk's server answered 403: This act needs the header x-atelier-desk: 1." });
    expect(succeeded(sent)).toBe(false);
  });

  it("reports a failure with no JSON, an answer it cannot read, and no connection", async () => {
    expect(await act(readyRequest("a", "m"), server(500, "oops").fetcher)).toEqual({ state: "failed", error: "The Desk's server answered 500." });
    expect(await act(readyRequest("a", "m"), server(200, { ok: true }).fetcher)).toEqual({ state: "failed", error: "The Desk's server sent an answer the page cannot read." });
    const offline = (async () => {
      throw new TypeError("Failed to fetch");
    }) as unknown as typeof fetch;
    expect(await act(readyRequest("a", "m"), offline)).toEqual({ state: "failed", error: "Cannot reach the Desk's server: TypeError: Failed to fetch" });
  });
});

describe("answers the Desk gave and no commit holds", () => {
  const q = (question: string, more: Partial<Question> = {}): Question => ({ item: "atelier-uwxy", question, finding: null, answer: null, outcome: null, ...more });

  it("marks the first unanswered question of the same item, text and finding", () => {
    const pending = q("Cache by HEAD?", { answer: { by: "sumeet", text: "Yes." } });
    const shown = withPending([q("Earlier?", { answer: { by: "x", text: "No." } }), q("Cache by HEAD?"), q("Cache by HEAD?")], [pending]);
    expect(shown.map((s) => s.pending)).toEqual([null, pending, null]);
  });

  it("matches nothing on another item or another finding", () => {
    const pending = q("Cache by HEAD?", { finding: "1-cccc.2", answer: { by: "sumeet", text: "Yes." } });
    const shown = withPending([q("Cache by HEAD?"), q("Cache by HEAD?", { item: "atelier-other", finding: "1-cccc.2" })], [pending]);
    expect(shown.map((s) => s.pending)).toEqual([null, null]);
  });

  it("matches two pending answers to two repeated questions in order", () => {
    const a = q("Again?", { answer: { by: "sumeet", text: "One." } });
    const b = q("Again?", { answer: { by: "sumeet", text: "Two." } });
    expect(withPending([q("Again?"), q("Again?")], [a, b]).map((s) => s.pending)).toEqual([a, b]);
  });
});
