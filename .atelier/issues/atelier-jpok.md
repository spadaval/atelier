---
created_at: "2026-10-17T01:10:57.991754064+00:00"
id: "atelier-jpok"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-jpok
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T07:14:18.880149998+00:00"
status: "done"
title: "Accept P0-P3 and priority words alike, and name stored tokens in errors"
updated_at: "2026-10-17T07:14:18.880149998+00:00"
---

## Goal

Records store `priority: "P2"` (`IssuePriority::canonical_token`, `crates/atelier-core/src/lib.rs:24-31`). The CLI accepts and prints only the words: `from_cli_input` calls `from_label` (`lib.rs:72`). As a result:
- `atelier issue create X -p P1` and `atelier issue list --priority P1` fail: `Invalid priority 'P1'. Valid values: critical, high, medium, low`.
- A hand-edited record with `priority: "low"` fails `atelier check` with `Invalid priority 'low'. Valid values: critical, high, medium, low`. The error lists the value it rejects, while the file actually needs `P3`.

An agent that reads record files, which are the durable boundary, gets refused by the CLI, and an agent that copies a CLI word into a file is refused by `check`.

## Done when

- `-p`/`--priority` on `issue create`, `issue update` and `issue list` accept both `P0`-`P3` and `critical/high/medium/low`.
- The record-parse error names the stored form: `priority 'low' in .atelier/issues/x.md: records store P0 (critical), P1 (high), P2 (medium) or P3 (low)`.
- Views keep printing the word, which is today's display. `docs/product/human-cli-output.md` or `work-model.md` states the mapping once.
- Tests: `-p P1` creates a P1 record; `--priority high` and `--priority P1` filter identically; a record containing `low` gives the new message.

## Context

`crates/atelier-core/src/lib.rs:54-97` (parsers and error Display). `work_classification.rs:385` already ranks both forms.

## Close Reason

Priority flags on issue create, update and list accept P0-P3 and the words alike, a record storing a word fails parsing with the stored tokens it needs, views still print the word, and the work model states the mapping.
