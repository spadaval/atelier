import { describe, expect, it } from "vitest";
import { countsLine, fetchedLine, finished, missionState, statusTitle, total } from "./words";

describe("counts", () => {
  it("spells the counts as atelier show names the statuses", () => {
    const counts = [
      { status: "draft" as const, count: 5 },
      { status: "in_review" as const, count: 1 },
      { status: "done" as const, count: 2 },
    ];
    expect(countsLine(counts)).toBe("5 draft, 1 in review, 2 done");
    expect(total(counts)).toBe(8);
    expect(finished(counts)).toBe(2);
    expect(countsLine([])).toBe("No items");
  });

  it("titles a status for a pill", () => {
    expect(statusTitle("changes_requested")).toBe("Changes requested");
    expect(statusTitle("ready")).toBe("Ready");
  });
});

describe("missions and fetches", () => {
  it("gives a mission's glyph from where it stands", () => {
    expect(missionState({ state: "proposed", in_progress: false })).toBe("draft");
    expect(missionState({ state: "open", in_progress: true })).toBe("building");
    expect(missionState({ state: "stopped", in_progress: false })).toBe("dropped");
  });

  it("says when a repository was fetched, how often, and why it last failed", () => {
    const now = Date.parse("2026-11-13T12:00:00Z");
    expect(fetchedLine({ remote: "origin", at: "2026-11-13T11:57:00Z", error: null, every_seconds: 300 }, now)).toBe(
      "Fetched from origin 3 minutes ago. The Desk fetches every 5 minutes.",
    );
    expect(fetchedLine({ remote: "origin", at: null, error: "no route", every_seconds: 0 }, now)).toBe(
      "Never fetched from origin. The Desk does not fetch it. Its last fetch failed: no route",
    );
    expect(fetchedLine({ remote: null, at: null, error: null, every_seconds: 300 }, now)).toMatch(/^This repository has no remote/);
  });
});
