---
created_at: "2026-06-12T21:42:51.151434546+00:00"
id: "atelier-jqds"
issue_type: "task"
labels:
- "stale-tests"
- "tests"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-4hec"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T22:08:59.363841831+00:00"
status: "done"
title: "Resolve obsolete ignored legacy command-surface tests"
updated_at: "2026-06-12T22:08:59.363841831+00:00"
---

## Description

The ignored-test inventory now exposes a legacy command-surface block where
tests still assert removed `history`, `session`, `timer`, `archive`,
`milestone`, and old import/export command behavior. Those assertions should
not be treated as normal product behavior while Atelier is targeting the new
CLI surface.
- Obsolete ignored tests for removed command surfaces are deleted or rewritten
  against the current product commands.
- Any intentionally retained ignored tests use the ignored-test metadata format
  with a reason, owner or linked issue, product-behavior classification, and
  blocking classification.
- The ignored-test inventory no longer reports the legacy command-surface block
  as a product-behavior closeout blocker.
- Run the ignored-test inventory or workflow validator and show no unresolved
  stale legacy command-surface blockers remain.
- Run focused tests for any rewritten command-surface behavior.
- Run `cargo fmt -- --check`, `target/debug/atelier export --check`, and
  `target/debug/atelier lint atelier-jqds`.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
