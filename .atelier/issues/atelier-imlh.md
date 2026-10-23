---
created_at: "2026-10-23T14:13:37.719873479+00:00"
id: "atelier-imlh"
issue_type: "feature"
labels:
- "cli-output"
priority: "P1"
relationships:
  blocks: []
  children:
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
status: "backlog"
title: "Gates name the approval they considered and why it is not current"
updated_at: "2026-10-23T14:13:48.168654560+00:00"
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
