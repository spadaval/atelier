---
created_at: "2026-10-17T01:10:57.975874283+00:00"
id: "atelier-cisa"
issue_type: "feature"
labels:
- "cli-output"
- "merge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "issue"
    id: "atelier-tciw"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Say which approval merge --check considered and why it is not current"
updated_at: "2026-10-23T15:32:35.601908114+00:00"
---

## Goal

`FAIL review.current: no current independent approval or explicit trivial exemption; authors: author a, author a <author@example.com>, author@example.com` is the same message whether there is no approval, the only approval came from an author, or the approval went stale after a later commit. `approval_is_current_to` (crates/atelier-app/src/review_gate.rs:169) finds the offending edge and then reduces it to a bool. The authors list also repeats one person in three normalized forms.

## Done when

- Every rejected approval or trivial-exemption candidate gets one line under the gate, for example:
  `⚠ atelier-x16l @e2562bd6 stale: base merge 4f1e2a3b changed in-scope record .atelier/issues/atelier-bw0b.md beyond status`
  `⚠ atelier-7l79 @4c79d30b stale: 1a2b3c4d "Tweak after approval" changed proof-sample.txt`
  `⚠ atelier-i3ay identity author@example.com matches a Git author`
- The FAIL line ends with the fix: `atelier review approve <id> --identity "<non-author>" "…"`.
- Authors are listed once per person (`Author A <author@example.com>`).
- The `merge_diverged` advice (crates/atelier-cli/src/commands/merge.rs:72) and docs/product/reviews-and-merge.md agree with the gate. They say a clean merge keeps the approval unless the base changed an in-scope record beyond status, and the stale line names that case. If the investigation shows the gate is wrong instead, fix the gate.
- Tests cover stale-by-code, stale-by-base-merge-of-in-scope-record and author-identity. `land` shows the same lines.

## Context

The real case: a clean `git merge master` into task/atelier-bw0b (master had edited .atelier/issues/atelier-bw0b.md) made approval atelier-x16l stale with no explanation. `allowed_base_merge` (review_gate.rs:194) correctly lets in-scope records change only in status. But the `merge_diverged` advice shown just before says "a clean merge keeps the approval", so the reader was told the opposite of what happened.

## Notes

- 2026-10-17 [note] Retro 2026-10-17: o3lm 06:25Z and 07:47Z, merge --check said 'resolution needs fresh independent approval' without naming the fix commit the approval predates; the orchestrator had to rediscover the re-approve step each time.
- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done only in merge --sync: merge_sync.rs:362-450 report_approval names the stale approval via approval_stale_edges (review_gate.rs:235). land and merge --check print only 'no current independent approval'. Author-identity rejection is silent everywhere (independent :688; report_approval skips it :399-400). 'A clean merge keeps the approval' is wrong in merge.rs:255,346,485 and session.rs:259 and human-cli-output.md:102.
- 2026-10-23 [decision] review.current names one approval: the newest stale independent approval or exemption, else the newest approval by an author identity
  Reason: The newest rejected candidate is what a reader acts on; older ones are stale for the same or more commits, and review list shows each event's standing.
  Rejected: One line per rejected candidate (the Done-when example), which repeats the same cause for every older approval
- 2026-10-23 [decision] The fix stays 'atelier review request <owner>' for every review.current cause, including author identity
  Reason: Every fix line must be a runnable command with real IDs; an approve command needs a reviewer identity the gate cannot know, and requesting review is the step the owner takes in each case.
  Rejected: atelier review approve <id> --identity "<non-author>" (a placeholder the mission's output rules forbid)
- 2026-10-23 [decision] merge --sync judges the approval by approval_standing (review.current's rule) instead of approval_accepted
  Reason: The sync promises to report by the gates' rule, and merge --check is the gate that follows it; approval_accepted is land's history rule and also counted findings, which findings.disposed reports.
  Rejected: Keeping approval_accepted in the sync, which could say 'Approval kept' while merge --check refused an author's approval
- 2026-10-23 [decision] approval_stale_edges reports a merge that brings in base commits the approval does not accept, even when the merge changes nothing against the branch
  Reason: A code conflict resolved to the branch's side made review.current fail while the stale list was empty, so it read as 'predates the fork'; which approvals count is unchanged.
