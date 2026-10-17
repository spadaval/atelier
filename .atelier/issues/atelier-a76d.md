---
created_at: "2026-10-17T01:10:57.990834507+00:00"
id: "atelier-a76d"
issue_type: "task"
labels:
- "cli-output"
- "help"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-a76d
priority: "P3"
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
status: "in_progress"
title: "Remove or implement --log-level and --log-format"
updated_at: "2026-10-17T06:29:35.185163574+00:00"
---

## Goal

Every help screen advertises `--log-level` and `--log-format` (`main.rs:69-81`), but the code has only two `tracing::warn!` sites (`commands/bundle.rs:737,1141`). `atelier --log-level trace work ready` writes nothing to stderr. Invalid values are silently accepted: `--log-level bogus status` and `--log-format xml status` exit 0, because `init_tracing` falls back to `warn` (main.rs:536). `docs/product/human-cli-output.md` promises diagnostic logging that doesn't exist.

## Done when

Either option:
- The flags are hidden or removed, and human-cli-output.md and development-setup.md say so.
- `--log-level debug` emits useful command context (record load, git calls, timings).

In both cases, an invalid value exits 2 with the allowed list (clap `value_parser`). A test covers the invalid value.

## Unknowns

Whether anything (scripts, hooks, `ATELIER_LOG` in adopters' environments) relies on the flags. Grep `scripts/` and astromech before removing them.
