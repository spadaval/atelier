import { describe, expect, it } from "vitest";
import type { ItemRow } from "../api/ItemRow";
import { groupByStatus, openByPriority, waitingOnYou } from "./group";

function row(id: string, status: ItemRow["status"], more: Partial<ItemRow> = {}): ItemRow {
  return {
    id,
    title: id,
    status,
    priority: "medium",
    labels: [],
    needs_human: false,
    startable: false,
    blockers: [],
    waiting_on: [],
    holder: null,
    reviewer: null,
    cannot_start: [],
    needs: [],
    branch: null,
    report_written: false,
    dropped: null,
    approved_at: null,
    ...more,
  };
}

describe("groupByStatus", () => {
  it("groups items in lifecycle order, keeps their order within a status and leaves empty statuses out", () => {
    const items = [row("a", "done"), row("b", "open"), row("c", "in_review"), row("d", "open"), row("e", "building"), row("f", "dropped")];
    expect(groupByStatus(items).map((g) => [g.status, g.items.map((i) => i.id)])).toEqual([
      ["open", ["b", "d"]],
      ["building", ["e"]],
      ["in_review", ["c"]],
      ["done", ["a"]],
      ["dropped", ["f"]],
    ]);
  });

  it("gives nothing for no items", () => {
    expect(groupByStatus([])).toEqual([]);
  });
});

describe("waitingOnYou", () => {
  it("lists open needs-human items", () => {
    const items = [row("c", "open", { needs_human: true }), row("d", "done", { needs_human: true }), row("e", "open")];
    expect(waitingOnYou(items).map((i) => i.id)).toEqual(["c"]);
  });
});

describe("openByPriority", () => {
  it("keeps only open items, the most urgent first and then by ID", () => {
    const items = [
      row("a", "open", { priority: "low" }),
      row("e", "open", { priority: "high" }),
      row("f", "building", { priority: "critical" }),
      row("d", "open", { priority: "medium" }),
      row("c", "open", { priority: "high" }),
      row("g", "open", { priority: "critical" }),
    ];
    expect(openByPriority(items).map((i) => i.id)).toEqual(["g", "c", "e", "d", "a"]);
  });
});
