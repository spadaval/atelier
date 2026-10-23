---
created_at: "2026-10-23T14:13:37.719873479+00:00"
id: "atelier-imlh"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-imlh
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3xq4"
  - kind: "issue"
    id: "atelier-cisa"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-h1nc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T15:46:27.656783466+00:00"
status: "done"
title: "Gates name the approval they considered and why it is not current"
updated_at: "2026-10-23T15:46:27.656783466+00:00"
---

## Goal

When an approval is not current, `land`, `merge --check` and `merge --sync` name the approval they considered and why it does not count: staled by a commit (with path), predates the fork, or recorded by an author identity. One function in `review_gate` produces the reason for all three.

## Done when

Tests for stale-by-code, stale-by-base-merge and author-identity cases assert the named approval and reason in land and `merge --check` output; `merge --sync` uses the same function. The four "a clean merge keeps the approval" advice strings and `docs/product/human-cli-output.md:102` are corrected. Delivers S2. Author proof plus independent adversarial review.

## Scope

Child: atelier-cisa. Output only; which approvals count does not change.

## Context

`merge_sync.rs:362-450` (`report_approval`) already explains staleness via `review_gate::approval_stale_edges` (`review_gate.rs:235`); move it into `review_gate` and use it from `evaluate_scope`'s `review.current` line, which the first rc8j increment renders.

## Unknowns

Final line shape depends on the renderer from the first rc8j increment.

## Close Reason

### Outcome
When an approval is not current, `land`, `merge`, `merge --check`, `merge --sync` and `review list` name the approval they considered and why it does not count: staled by a named commit and path, by a base merge that changed an in-scope record beyond status or was resolved by hand, predating the fork, or recorded by an author identity; all use one function, `review_gate::approval_standing` (atelier-cisa). The "a clean merge keeps the approval" advice now says when it holds. `merge --sync` rerun receipts carry the old Summary text with one rerun suffix, including for legacy nested receipts (atelier-3xq4). Delivers atelier-rc8j S2; shown by merge_gates tests for stale-by-code, stale-by-base-merge and author-identity in land and merge --check with every printed fix run verbatim (receipts atelier-uill, atelier-erv1, atelier-2vps, atelier-zx5z). Independent adversarial review found gate decisions identical to master across thirteen states.

### Deviations
The gate names the newest rejected approval rather than every one; `review list` lists all. The fix stays `atelier review request <owner>`. `merge --sync` reports the approval by review.current's rule, so it agrees with `merge --check`.

### Decisions
Decision notes on atelier-cisa and atelier-3xq4.

### Surprises
Stale-edge listing found nothing when a conflict was resolved to the branch's side; it now names the merge. Review found legacy rerun receipts would lose their summary (atelier-ytks); fixed in e5231071.

### Open items
Legacy rerun receipts' titles still hold the nested body (unchanged records; not rewritten by policy).
