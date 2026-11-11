import { describe, expect, it } from "vitest";
import type { LiveThread } from "../api/LiveThread";
import { threadsFor } from "./threads";
import { t3Note, threadStatus } from "./words";

function thread(id: string, holds: LiveThread["holds"]): LiveThread {
  return { id, title: id, status: "running", provider: "claudeAgent", model: "claude-opus-5-5", parent: null, pull_requests: [], link: null, updated_at: "2026-11-11T00:00:00Z", holds };
}

describe("the live threads a page lists", () => {
  const a = thread("a", [
    { repo: "atelier", kind: "item", id: "atelier-uwxy", role: "implementer" },
    { repo: "lor", kind: "mission", id: "lor-1", role: "lead" },
  ]);
  const free = thread("b", []);

  it("lists every thread on /agents, those holding nothing apart", () => {
    expect(threadsFor([a, free], undefined)).toEqual({ holding: [a], free: [free] });
  });

  it("lists on a repository's page only its threads, with only its holds", () => {
    const { holding, free: none } = threadsFor([a, free], "atelier");
    expect(holding.map((t) => t.holds)).toEqual([[{ repo: "atelier", kind: "item", id: "atelier-uwxy", role: "implementer" }]]);
    expect(none).toEqual([]);
    expect(threadsFor([a, free], "other").holding).toEqual([]);
  });
});

describe("what a page says about T3 Code", () => {
  it("says nothing when the last poll read the list", () => {
    expect(t3Note({ reachable: true, age_seconds: 3, error: null })).toBeNull();
  });

  it("says threads are unavailable when no list was ever read, with the reason", () => {
    expect(t3Note({ reachable: false, age_seconds: null, error: "The configuration has no [t3] table." })).toEqual({
      known: false,
      text: "Threads are unavailable: T3 Code cannot be reached.",
      error: "The configuration has no [t3] table.",
    });
  });

  it("gives the age of a list kept from before a failed poll", () => {
    expect(t3Note({ reachable: false, age_seconds: 130, error: null })).toMatchObject({ known: true, text: "T3 Code cannot be reached just now, so threads are shown as they were 2 minutes ago." });
  });

  it("names a thread waiting on a person, and an unknown status as it is", () => {
    expect(threadStatus("waiting")).toBe("Waiting on a person");
    expect(threadStatus("halted")).toBe("Halted");
  });
});
