import type { FindingRow } from "../api/FindingRow";
import type { ItemRow } from "../api/ItemRow";
import type { Question } from "../api/Question";
import type { Review } from "../api/Review";
import type { Status } from "../api/Status";
import { LIFECYCLE } from "./words";

export type StatusGroup = { status: Status; items: ItemRow[] };

/** Items under each status that has any, in lifecycle order, keeping their order within a status. */
export function groupByStatus(items: ItemRow[]): StatusGroup[] {
  return LIFECYCLE.map((status) => ({ status, items: items.filter((i) => i.status === status) })).filter((g) => g.items.length > 0);
}

/**
 * What waits on the human in a mission: its unanswered questions, then its
 * open items labelled needs-human that have none, each item once.
 */
export function waitingOnYou(items: ItemRow[], questions: Question[]): { questions: Question[]; items: ItemRow[] } {
  const open = questions.filter((q) => q.answer === null);
  const asked = new Set(open.map((q) => q.item));
  const closed = new Set<Status>(["done", "dropped"]);
  return { questions: open, items: items.filter((i) => i.needs_human && !closed.has(i.status) && !asked.has(i.id)) };
}

/**
 * The findings a review round asked to fix and that were fixed: fix-now,
 * no longer open, not withdrawn by the reviewer and not settled by a
 * decision to defer or take no action.
 */
export function fixedInReview(reviews: Review[]): { review: Review; finding: FindingRow }[] {
  return reviews.flatMap((review) =>
    review.findings
      .filter((f) => f.recommendation === "fix_now" && !f.open && f.now !== "withdrawn" && f.decided === null)
      .map((finding) => ({ review, finding })),
  );
}
