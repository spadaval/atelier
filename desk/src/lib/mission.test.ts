import { describe, expect, it } from "vitest";
import type { HistoryEvent } from "../api/HistoryEvent";
import type { ItemPage } from "../api/ItemPage";
import type { ItemRow } from "../api/ItemRow";
import { begun, logEntries, span, stage, stageAt } from "./mission";

const row = (id: string, status: ItemRow["status"], needs_human = false) => ({ id, status, needs_human }) as ItemRow;
const event = (at: string, kind: HistoryEvent["kind"], id: string, more: Partial<HistoryEvent> = {}): HistoryEvent => ({
  commit: "c",
  at,
  kind,
  id,
  record: "item",
  title: null,
  round: null,
  verdict: null,
  subject: "",
  ...more,
});
const page = (ready: ItemPage["ready"]) => ({ ready }) as ItemPage;

describe("an item's stage", () => {
  const events = [event("2026-11-16T10:00:00Z", "submitted", "a-0001"), event("2026-11-16T08:00:00Z", "submitted", "a-0001"), event("2026-11-16T07:00:00Z", "merged", "a-0002")];

  it("counts its review round from its submits", () => {
    expect(stage(row("a-0001", "in_review"), events)).toBe("in review 2");
    expect(stage(row("a-0003", "in_review"), events)).toBe("in review 1");
    expect(stage(row("a-0002", "done"), events)).toBe("merged");
    expect(stage(row("a-0004", "done"), events)).toBe("done");
    expect(stage(row("a-0005", "changes_requested"), events)).toBe("changes req");
  });

  it("dates it from its latest step, else from when it was readied", () => {
    expect(stageAt("a-0001", events, null)).toBe("2026-11-16T10:00:00Z");
    expect(stageAt("a-0009", events, page({ by: "ada", at: "2026-11-15T00:00:00Z" }))).toBe("2026-11-15T00:00:00Z");
    expect(stageAt("a-0009", events, null)).toBeNull();
  });

  it("spells a duration short", () => {
    expect(span(48_000)).toBe("48s");
    expect(span(36 * 60_000)).toBe("36m");
    expect(span((2 * 60 + 14) * 60_000)).toBe("2h 14m");
    expect(span(72 * 3_600_000)).toBe("3d");
  });
});

describe("the progress log", () => {
  const items = [row("a-0001", "building"), row("a-0002", "open", true), row("a-0003", "open")];
  const events = [
    event("2026-11-16T10:00:00Z", "started", "a-0001"),
    event("2026-11-16T09:00:00Z", "readied", "a-0001"),
    event("2026-11-16T08:30:00Z", "review", "a-0002", { round: 1, verdict: "changes requested" }),
    event("2026-11-16T08:00:00Z", "merged", "m-0001", { record: "mission" }),
  ];
  const pages = new Map([
    ["a-0001", page({ by: "ada, in conversation on 2026-11-16: go", at: "2026-11-16T09:00:00Z" })],
    ["a-0003", page({ by: "ada, in conversation on 2026-11-16: go", at: "2026-11-16T09:00:20Z" })],
    ["a-0002", page({ by: "conversion", at: "2026-11-16T07:00:00Z" })],
  ]);

  it("names the steps newest first, and marks what waited on the human", () => {
    const log = logEntries("m-0001", items, events, pages, "Ada");
    expect(log.map((e) => [e.text, e.waited])).toEqual([
      ["Started a-0001", false],
      ['Ada readied 2 items: "go"', true],
      ["Review 1 of a-0002: changes requested", true],
      ["Plan merged: mission agreed", false],
    ]);
  });

  it("counts no ready mark as the human's while the caller is not known", () => {
    const log = logEntries("m-0001", items, events, pages, "");
    expect(log.map((e) => e.text)).toEqual(["Started a-0001", "Readied a-0001", "Review 1 of a-0002: changes requested", "Plan merged: mission agreed"]);
  });
});

describe("a mission under way", () => {
  it("is one with an item started or done", () => {
    const counts = (...list: [ItemRow["status"], number][]) => list.map(([status, count]) => ({ status, count }));
    expect(begun({ in_progress: true, counts: counts(["open", 2]) })).toBe(true);
    expect(begun({ in_progress: false, counts: counts(["open", 2], ["done", 1]) })).toBe(true);
    expect(begun({ in_progress: false, counts: counts(["open", 3], ["dropped", 1]) })).toBe(false);
  });
});
