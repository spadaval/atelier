---
created_at: "2026-06-21T22:49:37.219532601+00:00"
id: "atelier-fsg2"
issue_type: "task"
labels:
- "cleanup"
- "mission-rework"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-53bu"
    type: "related"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-22T14:49:11.104802869+00:00"
status: "done"
title: "Clean mission rework fallout"
updated_at: "2026-06-22T14:49:11.104802869+00:00"
---

## Description

Remove remaining live first-class mission record/code/test/docs residue after atelier-53bu. The public root `atelier mission` command is already gone; this cleanup should align implementation helpers, tests, recovery hints, and target-state docs with issue-backed mission objectives. Historical closed tracker activity may remain historical, but live code, tests, current docs, and non-terminal tracker guidance should not teach removed mission commands or `.atelier/missions` as target state.

## Outcome

- Live code no longer preserves first-class mission storage helpers or recovery hints for removed `atelier mission` commands except where explicitly needed to read historical/imported data.
- Tests no longer assert canonical `.atelier/missions` or `schema: "atelier.mission"` as the target contract.
- Product/spec/Agent guidance no longer contradicts the issue-backed mission objective model.
- Full relevant Rust validation is no longer blocked by stale mission-record tests.

## Evidence

- `rg` transcript shows only historical/import/migration references for removed mission commands and first-class mission storage.
- `cargo fmt -- --check`, `cargo nextest run`, `target/debug/atelier lint`, and `git diff --check` pass, or any residual failure is tracked with a follow-up issue.
