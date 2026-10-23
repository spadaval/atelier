---
created_at: "2026-10-23T14:13:37.951722432+00:00"
id: "atelier-l7nz"
issue_type: "feature"
labels:
- "cli-output"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-l7nz
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
closed_at: "2026-10-23T16:27:43.465229895+00:00"
status: "done"
title: "Evidence record, list and show: one line, by item, brief"
updated_at: "2026-10-23T16:27:43.465229895+00:00"
---

## Goal

`evidence record` confirms in one line ending in its verdict, with the failure tail on a fail; `evidence list <item>` filters by item with fresh, stale and superseded marks and a limit; `evidence show` is brief with `--full` for the transcript; evidence commands accept one target syntax.

## Done when

CLI tests pin record's pass and fail lines, list by item with marks and `--limit`/`--all`, brief and full show, and `evidence attach <id> issue/<id>` plus bare-ID targets. Delivers the remaining S3 clauses. Author proof plus independent review.

## Scope

Children: atelier-3ekh, atelier-o8ni, atelier-u0ig, atelier-b05y. One freshness-mark helper over `evidence_freshness` serves list and show. Quiet `evidence record` stays the record's ID (human-cli-output.md quiet-token table).

## Context

Regroom 2026-10-23 notes. `crates/atelier-cli/src/commands/evidence.rs` (`print_new_record` :761, `print_record_with_targets` :771, `list` :660-727), `crates/atelier-app/src/evidence_freshness.rs:58,190`. Builds on the receipt shape from the earlier evidence increment.

## Close Reason

### Outcome
`evidence record` confirms in one line with the receipt's result, target, pin and command, adds the failure tail on a fail, and ends with its verdict (`evidence record: passed`, or `recorded fail — <item> cannot close or merge until a later pass`, exit 0) (atelier-3ekh). `evidence list [item]` filters by item, newest first, marks each receipt as the gates judge it (fresh, stale with the commit, superseded, blocks completion, or held at close for a merged claim), takes `--limit` and `--all`, and ends with a `Proof:` line (atelier-u0ig). `evidence show` is brief, with `--full` for the stored record (atelier-o8ni). `attach` and `--target` accept `issue/<id>` and bare IDs, and an unknown `--kind` warns (atelier-b05y). Completes atelier-rc8j S3 with atelier-dg3h; shown by CLI tests in records_evidence.rs and session.rs, including list/gate agreement and a merged claim judged as land and doctor judge it (receipts atelier-ypyf, atelier-4ab7 and focused receipts on each child).

### Deviations
The mark sits beside the result word. A capture refused before running prints no verdict line yet (no check codes for those refusals). The views share `item_proof`, but the gates keep their own supersession code (follow-up atelier-4urj).

### Decisions
Decision notes on each child; recorded-fail exit status per atelier-rc8j's decision.

### Surprises
Review found merged items' passes marked stale while land and doctor accepted them (atelier-2isj), and a manual pass on a dirty checkout confirmed as passed while the list said stale (atelier-g2pl); both fixed in 2148b5a5.

### Open items
Check codes for capture refusals, so they can end with a refused verdict.
