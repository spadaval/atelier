import { describe, expect, it } from "vitest";
import type { ItemRow } from "../api/ItemRow";
import type { Question } from "../api/Question";
import { groupByStatus, waitingOnYou } from "./group";

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
    question: null,
    report_written: false,
    dropped: null,
    approved_at: null,
    ...more,
  };
}

describe("groupByStatus", () => {
  it("groups items in lifecycle order, keeps their order within a status and leaves empty statuses out", () => {
    const items = [row("a", "done"), row("b", "draft"), row("c", "in_review"), row("d", "draft"), row("e", "building"), row("f", "dropped")];
    expect(groupByStatus(items).map((g) => [g.status, g.items.map((i) => i.id)])).toEqual([
      ["draft", ["b", "d"]],
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
  it("lists unanswered questions, then open needs-human items without one, each item once", () => {
    const questions: Question[] = [
      { item: "a", question: "Which?", finding: null, answer: null, outcome: null },
      { item: "b", question: "Done?", finding: null, answer: { by: "sumeet", text: "Yes" }, outcome: null },
    ];
    const items = [row("a", "building", { needs_human: true }), row("c", "ready", { needs_human: true }), row("d", "done", { needs_human: true }), row("e", "ready")];
    const waiting = waitingOnYou(items, questions);
    expect(waiting.questions.map((q) => q.item)).toEqual(["a"]);
    expect(waiting.items.map((i) => i.id)).toEqual(["c"]);
  });
});
