---
created_at: "2026-10-17T01:10:57.975561669+00:00"
id: "atelier-b05y"
issue_type: "task"
labels:
- "cli-output"
- "evidence"
priority: "P3"
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
status: "backlog"
title: "Use one evidence target syntax and a result token in quiet output"
updated_at: "2026-10-23T14:14:22.951760692+00:00"
---

## Goal

Small inconsistencies in the evidence commands:
- `evidence record` takes `--target issue/<id>`, but `evidence attach` wants `<ID> <TARGET_KIND> <TARGET_ID>` and rejects `atelier evidence attach atelier-4tb7 issue/atelier-l36s` with a missing-argument error.
- `--kind` accepts any string (`--kind weird` is recorded without a word), so a typo creates a new kind that no gate or filter matches.
- Errors still name flags that no longer exist: `--target-kind and --target-id must be supplied together` (evidence.rs:135, 202, 298, 331).
- `--quiet` prints `atelier-n3tn recorded test exit=1 target=issue/atelier-nqgr(validates)` (evidence.rs:666). `recorded` is the record status and is the same on every receipt, while the pass/fail result is missing. That goes against human-cli-output.md ("`--quiet` limits output to … result tokens").

## Done when

- `evidence attach <id> issue/<id>` works, and the two-argument form is still accepted.
- An unknown `--kind` warns and lists the known kinds (or the kinds in use) without refusing.
- No error mentions `--target-kind` or `--target-id`.
- Quiet `evidence record` and `evidence show` print `<id> <result>` (for example `atelier-n3tn fail`).
- Tests cover each item.

## Notes

- 2026-10-17 [note] Retro 2026-10-17: `evidence record --target atelier-sdb0` failed with '--target must use kind/id syntax' while every other command takes a bare id (1w8c, 02:53Z).
- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: quiet evidence show prints '<id> <result>'. Drop the quiet 'evidence record -> <id> <result>' clause: the quiet-token table fixes it as the ID. Still: attach rejects issue/<id>; --kind weird accepted silently; stale --target-kind/--target-id errors at evidence.rs:216,283,400,444; bare --target <id> rejected.
