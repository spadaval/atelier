// The debrief's wording, as `atelier debrief` prints it
// (crates/atelier-cli/src/commands/mission_review.rs). The server computes
// the view; these only put its facts into words.

import type { Ask } from "./api/Ask";
import type { Change } from "./api/Change";
import type { CloseReport } from "./api/CloseReport";
import type { CriterionMark } from "./api/CriterionMark";
import type { CriterionProof } from "./api/CriterionProof";
import type { Debrief } from "./api/Debrief";
import type { MalformedDecision } from "./api/MalformedDecision";
import type { NoteDecision } from "./api/NoteDecision";
import type { OpenEntry } from "./api/OpenEntry";
import type { ReportPart } from "./api/ReportPart";
import type { ReviewSummary } from "./api/ReviewSummary";
import type { ValidationRun } from "./api/ValidationRun";
import type { WorkEntry } from "./api/WorkEntry";
import { countNoun, firstSentence, localDate, shortCommit } from "./text";

/** A report part, unless it says only "none" or "n/a". */
export function reported(report: CloseReport | null, part: ReportPart): string | null {
  const text = report?.parts.find(([name]) => name === part)?.[1];
  if (!text) return null;
  const word = text.trim().replace(/\.+$/, "").toLowerCase();
  return word === "none" || word === "n/a" ? null : text;
}

export function changedCourse(entry: WorkEntry): boolean {
  return (
    entry.changes.length > 0 ||
    [entry.report, ...entry.children.map((child) => child.report)].some(
      (report) => reported(report, "deviations") !== null,
    )
  );
}

export function criterionWord(mark: CriterionMark): string {
  return { proven: "✓ proven", failing: "⚠ failing", unproven: "unproven" }[mark];
}

export function commitsSince(count: number, closed: boolean): string {
  return `${countNoun(count, "code commit")} ${closed ? "before close" : "since"}`;
}

/** The deciding receipt and its age. */
export function proofCue(proof: CriterionProof, closed: boolean): string | null {
  const receipt = proof.receipts.find((receipt) => receipt.in_history);
  if (!receipt) return null;
  let cue = `${receipt.evidence_id} ${receipt.result} at ${shortCommit(receipt.commit)}`;
  if (receipt.code_commits_since) cue += ` | ${commitsSince(receipt.code_commits_since, closed)}`;
  return cue;
}

export function changeText(change: Change): string {
  switch (change.kind) {
    case "reopened":
      return `Reopened on ${localDate(change.value)} after a done claim.`;
    case "non_pass_evidence":
      return `Newest proof ${change.value.evidence} is ${change.value.result}: ${firstSentence(change.value.summary)}`;
    case "trivial_exemption":
      return `Review waived as trivial: ${firstSentence(change.value.summary)}`;
    case "done_when_edited": {
      const edit = change.value;
      const after =
        edit.after === null
          ? ""
          : edit.after.kind === "finding"
            ? `, after finding ${edit.after.value}`
            : `, after failing receipt ${edit.after.value}`;
      return `${edit.issue} Done when edited at ${shortCommit(edit.commit)} after the item was started${after}.`;
    }
  }
}

/** `7 findings: 3 fixed, 4 deferred`, or null without findings. */
export function findingCounts(review: ReviewSummary): string | null {
  const parts: [number, string][] = [
    [review.fixed, "fixed"],
    [review.deferred, "deferred"],
    [review.no_action, "left as is"],
    [review.wont_fix, "won't fix"],
    [review.undisposed, "awaiting a decision"],
  ];
  const findings = parts.reduce((sum, [count]) => sum + count, 0);
  if (findings === 0) return null;
  return `${countNoun(findings, "finding")}: ${parts
    .filter(([count]) => count > 0)
    .map(([count, word]) => `${count} ${word}`)
    .join(", ")}`;
}

export function reviewSummary(review: ReviewSummary): string {
  const verdict = review.approved
    ? review.approvers.length === 0
      ? "Approved"
      : `Approved by ${review.approvers.join(", ")}`
    : review.trivial
      ? "Waived as trivial"
      : "Not approved";
  const counts = findingCounts(review);
  return counts === null ? `${verdict}; no findings.` : `${verdict}; ${counts}.`;
}

/** How review went; null for a decision record and for declined work nobody reviewed. */
export function reviewLine(view: Debrief, entry: WorkEntry): string | null {
  const issue = view.refs[entry.id];
  if (issue?.issue_type === "decision") return null;
  if (!entry.review.recorded) {
    if (issue?.status === "declined") return null;
    return "No review recorded.";
  }
  return reviewSummary(entry.review);
}

export function titleOf(view: Debrief, id: string): string {
  return view.refs[id]?.title ?? id;
}

function criterionText(view: Debrief, slug: string): string {
  const proof = view.criteria.proofs.find((proof) => proof.criterion.slug === slug);
  return proof ? firstSentence(proof.criterion.text) : "";
}

/** An ask's title and detail. */
export function askLine(view: Debrief, ask: Ask): [string, string] {
  const summary = (id: string) => view.refs[id]?.summary ?? "";
  switch (ask.kind) {
    case "decision":
      return [`Decide: ${titleOf(view, ask.value)}`, summary(ask.value)];
    case "needs_human":
      return [titleOf(view, ask.value), summary(ask.value)];
    case "failing":
      return [`Criterion ${ask.value} is failing`, criterionText(view, ask.value)];
    case "unproven": {
      const slugs = ask.value;
      return [
        `${slugs.length === 1 ? "1 criterion" : `${slugs.length} criteria`} without passing proof: ${slugs.join(", ")}`,
        slugs.length === 1 ? criterionText(view, slugs[0]) : "",
      ];
    }
  }
}

export function awaitedBy(view: Debrief, slug: string): string | null {
  return view.validation.awaiting.find(([awaiting]) => awaiting === slug)?.[2] ?? null;
}

export function statusWord(view: Debrief): string {
  if (view.status_category === "done") return "Done";
  if (view.mission.status === "draft") return "Draft";
  return "In progress";
}

/** The verdict the debrief opens with. */
export function verdict(view: Debrief): string[] {
  const parts: string[] = [];
  const total = view.criteria.proofs.length;
  if (total > 0) {
    const proven = view.criteria.proofs.filter((proof) => proof.mark === "proven").length;
    parts.push(`${proven} of ${total} criteria proven`);
  }
  if (view.asks.length > 0) parts.push(view.asks.length === 1 ? "1 needs your review" : `${view.asks.length} need your review`);
  if (view.left.length > 0) parts.push(`${view.left.length} left to do`);
  if (view.found.length > 0) parts.push(`${view.found.length} found along the way`);
  return parts;
}

/** Each criterion a run judged, with its result: `S1 pass, S2 fail`. */
export function judged(run: ValidationRun): string {
  return run.receipts
    .flatMap((receipt) => receipt.criteria.map((criterion) => `${criterion.slug} ${criterion.result}`))
    .join(", ");
}

export function lastDelivery(view: Debrief): string | null {
  const times = view.work.map((entry) => entry.closed_at).filter((at): at is string => at !== null);
  if (times.length === 0) return null;
  return times.reduce((a, b) => (new Date(a) >= new Date(b) ? a : b));
}

export function origin(entry: OpenEntry): [string, string] | null {
  return entry.from === null ? null : [entry.from, entry.discovered ? "found in" : "from review of"];
}

export function decisionText(note: NoteDecision): string {
  return `${note.decision.choice} | because ${note.decision.reason}`;
}

export function malformedText(note: MalformedDecision): string {
  return `${note.issue_id} #${note.number} ${note.defect}`;
}

/** The decision notes and malformed notes on one item, as labelled lines. */
export function decisionsOn(view: Debrief, id: string): [string, string][] {
  return [
    ...view.decisions.notes
      .filter((note) => note.issue_id === id)
      .map((note): [string, string] => ["Decided", decisionText(note)]),
    ...view.decisions.malformed
      .filter((note) => note.issue_id === id)
      .map((note): [string, string] => ["⚠ Malformed decision note", malformedText(note)]),
  ];
}

/** A done child's report as labelled facts. */
export function reportFacts(report: CloseReport | null, openItems: boolean): [string, string][] {
  const facts: [string, string][] = [];
  if (report?.outcome) facts.push(["Outcome", report.outcome]);
  const parts: [string, ReportPart][] = [
    ["Changed", "deviations"],
    ["Surprised by", "surprises"],
    ["Decided", "decisions"],
    ["Left open", "open_items"],
  ];
  for (const [label, part] of parts) {
    if (part === "open_items" && !openItems) continue;
    const text = reported(report, part);
    if (text !== null) facts.push([label, text]);
  }
  return facts;
}

/** A delivered entry's facts, in the order the CLI prints them. */
export function deliveredFacts(view: Debrief, entry: WorkEntry): [string, string][] {
  const facts: [string, string][] = [];
  const fact = (label: string, text: string | null) => {
    if (text !== null) facts.push([label, text]);
  };
  fact("Changed", reported(entry.report, "deviations"));
  for (const change of entry.changes) fact("Changed", changeText(change));
  fact("Surprised by", reported(entry.report, "surprises"));
  fact("Decided", reported(entry.report, "decisions"));
  for (const note of entry.decisions) fact("Decided", decisionText(note));
  for (const note of entry.malformed) fact("⚠ Malformed decision note", malformedText(note));
  const record = view.decisions.records.find((record) => record.issue.id === entry.id);
  if (record) {
    if (record.issue.adr) fact("ADR", record.issue.adr.path);
    for (const id of record.supersedes) fact("Replaces", itemText(view, id));
  }
  fact("Review", reviewLine(view, entry));
  for (const id of entry.follow_ups) fact("Filed", itemText(view, id));
  if (entry.follow_ups.length === 0) fact("Left open", reported(entry.report, "open_items"));
  return facts;
}

/** An item as text: id, status and title. */
export function itemText(view: Debrief, id: string): string {
  const issue = view.refs[id];
  return issue ? `${issue.id} ${issue.status} ${issue.title}` : id;
}
