---
created_at: "2026-06-20T15:11:13.566965565+00:00"
id: "atelier-0rx5"
issue_type: "task"
labels:
- "cutting-pass"
- "forgejo"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Move review-link validation into one app-layer contract"
updated_at: "2026-06-20T16:49:00.525133989+00:00"
---

## Description

The audit found Forgejo review validation split between app and CLI layers.
`crates/atelier-cli/src/commands/pr.rs` also repeats room/provider branching
across open, link, status, show, merge, comments, review, and resolve paths.
Consolidate linked review-artifact parsing, provider/room routing, and
merge-readiness checks into one app-layer contract, then have CLI workflow and
PR commands call that contract.

## Outcome

Review and Forgejo validation behavior has one source of truth outside command
dispatch. CLI transition and PR commands no longer re-parse provider review
fields or repeat room/provider routing independently from the app layer.

## Evidence

- `rg` shows review-link parsing, provider/room routing, and linked PR
  merge-readiness logic is centralized in the app layer.
- Workflow transition tests still cover `review.complete`, `review.merge`, and missing/malformed linked review artifact cases.
- `cargo test -p atelier-cli` focused review/Forgejo filters pass.
