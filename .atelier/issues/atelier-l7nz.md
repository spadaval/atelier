---
created_at: "2026-10-23T14:13:37.951722432+00:00"
id: "atelier-l7nz"
issue_type: "feature"
labels:
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3ekh"
  - kind: "issue"
    id: "atelier-b05y"
  - kind: "issue"
    id: "atelier-o8ni"
  - kind: "issue"
    id: "atelier-u0ig"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dg3h"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Evidence record, list and show: one line, by item, brief"
updated_at: "2026-10-23T14:47:59.283581928+00:00"
---

## Goal

`evidence record` confirms in one line ending in its verdict, with the failure tail on a fail; `evidence list <item>` filters by item with fresh, stale and superseded marks and a limit; `evidence show` is brief with `--full` for the transcript; evidence commands accept one target syntax.

## Done when

CLI tests pin record's pass and fail lines, list by item with marks and `--limit`/`--all`, brief and full show, and `evidence attach <id> issue/<id>` plus bare-ID targets. Delivers the remaining S3 clauses. Author proof plus independent review.

## Scope

Children: atelier-3ekh, atelier-o8ni, atelier-u0ig, atelier-b05y. One freshness-mark helper over `evidence_freshness` serves list and show. Quiet `evidence record` stays the record's ID (human-cli-output.md quiet-token table).

## Context

Regroom 2026-10-23 notes. `crates/atelier-cli/src/commands/evidence.rs` (`print_new_record` :761, `print_record_with_targets` :771, `list` :660-727), `crates/atelier-app/src/evidence_freshness.rs:58,190`. Builds on the receipt shape from the earlier evidence increment.
