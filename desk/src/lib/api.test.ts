import { afterEach, describe, expect, it, vi } from "vitest";
import { refresh } from "./api";

/** A fetch whose answers the test releases one at a time, each the server's state when the read started. */
function server() {
  let state = "before";
  const reads: { saw: string; answer: () => void }[] = [];
  vi.stubGlobal("fetch", (path: string) => {
    const saw = state;
    return new Promise<Response>((resolve) => reads.push({ saw, answer: () => resolve(new Response(JSON.stringify({ saw }), { status: 200 })) }));
  });
  return { reads, change: () => (state = "after") };
}

const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

afterEach(() => vi.unstubAllGlobals());

describe("reading a page again", () => {
  it("answers a read asked for during another with one that started after it, shared by every such caller", async () => {
    const { reads, change } = server();
    const path = "/api/test/during";
    const first = refresh(path);
    expect(reads).toHaveLength(1);
    // A commit lands while the first read is under way, and two polls ask again.
    change();
    const again = refresh(path);
    const andAgain = refresh(path);
    expect(again).toBe(andAgain);
    reads[0].answer();
    await first;
    await settle();
    expect(reads.map((r) => r.saw)).toEqual(["before", "after"]);
    reads[1].answer();
    await again;
    expect(reads).toHaveLength(2);
  });

  it("starts a read at once when none is under way", async () => {
    const { reads } = server();
    const path = "/api/test/idle";
    const done = refresh(path);
    expect(reads).toHaveLength(1);
    reads[0].answer();
    await done;
    const next = refresh(path);
    expect(reads).toHaveLength(2);
    reads[1].answer();
    await next;
  });
});
