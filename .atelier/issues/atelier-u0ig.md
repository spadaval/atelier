---
created_at: "2026-10-17T01:10:57.974957341+00:00"
id: "atelier-u0ig"
issue_type: "feature"
labels:
- "cli-output"
- "evidence"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Filter evidence list by item and show freshness and a limit"
updated_at: "2026-10-23T14:47:59.047316469+00:00"
---

## Goal

No view answers "what proof does atelier-nqgr have, and is it still fresh?". `atelier evidence list` has only `--result` and a fixed limit of 20 (crates/atelier-cli/src/commands/evidence.rs:19) with no flag to see more: `Omitted: 181 older evidence record(s) hidden by default limit 20`. It marks a receipt `pass` even after a later code commit made it stale, which docs/product/human-cli-output.md forbids ("Never present an attached receipt as passing proof without checking its result and freshness"). The header states the count three times (`201 total` / `Showing: 20 of 201` / `Omitted: 181`). Rows repeat the default `(validates)`, print `exit (none) … command (manual)` for manual proof, use the placeholder `(command-backed proof)`, and cut commands to three words (`cargo nextest run ...`).

Agents work around it. In astromech, 5 of 6 `evidence list` runs were `| grep <id>`. The cross-cutting audit saw 5 attempts at `evidence list atelier-bibd` and `evidence list --target issue/ID` (all errors) and about 150 direct reads of `.atelier/evidence`.

## Done when

- `atelier evidence list atelier-nqgr` (positional, short IDs OK) lists that item's receipts newest first. Each row is marked `✓ fresh`, `⚠ stale (code changed in 1a2b3c4d)` or `⚠ superseded by newer fail atelier-n3tn`, using the same freshness logic as the `evidence.*` gates. The list ends with a verdict line, for example `Proof: ⚠ newest is fail atelier-n3tn`.
- `--limit N` and `--all` exist. The count appears once. Rows drop default and `(none)` tokens and show the full command up to the width limit.
- The "List matching IDs" footer keeps the active filters.
- Tests cover filtering, short IDs, stale and superseded marks, and `--limit`.

## Context

atelier-sdb0 shows owner evidence freshness in status. atelier-et6q will name criteria on evidence, and this view should leave room for them.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. List has only --result (main.rs:684); fixed limit 20 (evidence.rs:20); count printed 3 times (:660-695); no freshness mark. Use evidence_freshness (evidence_is_fresh :58, newest_failure :190) in one helper shared with o8ni.
