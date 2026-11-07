---
created_at: "2026-10-30T06:40:25.029767683+00:00"
id: "atelier-r9hb"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-r9hb
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T00:07:32.475021384+00:00"
status: "done"
title: "Show a Done when edited after the item was started"
updated_at: "2026-11-07T00:07:32.475021384+00:00"
---

## Goal

When an item's Done when is edited after the item was started, show the edit where closing is judged, so a criterion loosened to let an item close is seen by the reviewer and the human rather than hidden by a fresh approval. In the usage review most narrowings had no recorded finding before them, so the trigger is the start, not a finding.

## Done when

Tests show: for an item started at commit S, with its Done when edited at commit B that descends from S (Git ancestry, not timestamps) and an approval after B, `merge <owner> --check` still passes, with an unchanged exit status, but prints one line naming the item and commit B, and also the finding or failing receipt on the item pinned to an ancestor of B, when there is one. `atelier debrief <mission>` lists the same edit under the item. An item whose Done when changed only before S shows no line. Reverting the detection makes these tests fail.

## Scope

`merge --check` and `merge` output, the debrief model (crates/atelier-app/src/mission_review.rs) and its renderers, docs/product/reviews-and-merge.md. The verdict and exit status do not change. Mission success criteria are out of scope: their failing receipts sit on validation items and findings are rarely recorded on missions.

## Context

Record history from Git: crates/atelier-app/src/record_history.rs (`issue_histories`). Order edits, findings and receipts by Git ancestry of their commits; the debrief already mixes clocks (atelier-b3qt).

## Design

Surface, do not refuse. A Done when is sometimes rightly corrected after a finding, and refusing would need an override or a human-only record, which ADR 0018 keeps Atelier from becoming. The line gives the reviewer and the debrief reader the fact to judge.

## Notes

- 2026-11-06 [note] Taken out from under atelier-ma4a on 2026-11-06 so it can be started and merged on its own branch: atelier-n0qh (mission atelier-n4mj) is blocked by it, and atelier-ll3p is still a draft and has not scheduled it. docs/strategies/lighter-workflow.md (Risks) lets atelier-n4mj's orchestrator do r9hb as that mission's work while leaving it in atelier-ll3p; it still advances atelier-ll3p, and atelier-ma4a's Done when covers its remaining children.
- 2026-11-06 [decision] Read every non-merge commit that touched the record (git log --full-history --no-merges), not the first-parent walk issue_histories makes, and judge 'after the start' as: some commit that moved the item into an active status is an ancestor of the edit's parent.
  Reason: Under merge_commit the base's first-parent line folds a branch's start and edit into one merge, so the walk debrief already uses cannot place an edit after a start; the commit that wrote the edit is always in the history, and the Done when asks for Git ancestry, not timestamps.
  Rejected: Reuse issue_histories positions: collapses branch history on the base
- 2026-11-06 [decision] Name one precedent: the newest (by record time) finding or fail receipt on the item whose pin is an ancestor of the edit; blocked receipts and findings on a parent owner do not count. merge --json is left unchanged.
  Reason: The Done when names one finding or failing receipt on the item; ancestry decides eligibility, the record clock only picks among eligible ones. --json is a script format (ADR 0022) whose node list would need a new shape for a non-gate fact.
  Rejected: List every eligible finding and receipt: noisier line for the same judgement

## Close Reason

`merge` and `merge --check` now print `Note: <id> Done when edited at <commit> after the item was started[, after finding <id> | after failing receipt <id>].` for each Done when edit, in the owner or a scoped child, whose commit descends in Git from a commit that started the item. The debrief lists the same fact under the item as a "Changed" line. Verdict and exit status are unchanged, and an edit made only before the start shows nothing.

Targeted tests, all passing: merge_gates `a_done_when_edited_after_the_start_is_shown_without_changing_the_verdict` (check and merge pass with exactly one line naming the item, commit B and the finding; the Markdown and HTML debrief show it), `a_done_when_edited_after_a_failing_receipt_names_the_receipt`, `a_done_when_edited_only_before_the_start_is_not_shown`, and cli_integration `debrief::a_done_when_edit_counts_as_after_the_start_by_ancestry_not_time` (a later-by-clock base edit across a `--no-ff` merge is not shown). Each reversion was run and made tests fail. Dropping the start check failed the before-start test. Disabling detection failed both positive merge tests. Replacing the ancestry check with "any start exists" failed the ancestry test. The full `cargo nextest run -p atelier-app -p atelier-cli` passed (865 tests), and `cargo fmt -- --check` and `git diff --check` are clean.

### Deviations
`merge --json` does not carry the note. `-q` sends it to stderr so stdout keeps its `passed` token. The docs also changed docs/product/human-cli-output.md (Gate Lines) and docs/product/mission-reviews.md, besides reviews-and-merge.md.

### Decisions
See the two decision notes on this item: a full non-merge history walk judged by ancestry instead of issue_histories, and a single newest eligible precedent with JSON left alone.

### Surprises
The repo's `merge_commit` strategy means the existing first-parent record walk (issue_histories) cannot place a branch's start before its edit on the base, so the detection walks the full history itself.

### Open items
- A Done when changed only inside a merge commit's conflict resolution is not shown, because merges are not read.
- "Started" means a move into an `active` status category (`in_progress` without a policy). An item closed without ever being active would show no edits; the shipped workflow cannot do that.
- Mission success criteria are out of scope, as the item says.
- Independent review is outstanding.
