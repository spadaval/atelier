---
created_at: "2026-10-17T01:10:57.988678443+00:00"
id: "atelier-7w1g"
issue_type: "task"
labels:
- "cli-output"
- "help"
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
status: "todo"
title: "Document every argument and flag in subcommand help"
updated_at: "2026-10-17T07:20:06.440307265+00:00"
---

## Goal

These print with empty descriptions in `--help`:
- `merge <ID>`, `--check`, `--json`
- `work mission <ID>`, `work parent <ID>`
- `bundle preview/apply <INPUT>`, `bundle apply --yes`
- `evidence record --kind`, `--result` (values not listed), `--summary`, `--path`, `--uri`, `--producer`, `--role`, `[COMMAND]...`
- `evidence show <ID>`; `evidence attach <ID> <TARGET_KIND> <TARGET_ID>`, `--role`
- `evidence list --result`

`issue list --status/--category` and `issue create --status` don't name allowed values. Global options are interleaved with command options (`init --help`: `-f`, `-q`, `--log-level`, `--slug`, `--log-format`). Agents strip them with `grep -vE 'quiet|log-'` when reading help (4 transcript runs).

## Done when

- Every argument and flag has a one-line description.
- Enumerated values show as `[possible values: ...]` or, for status and category, the workflow's values.
- Global options sort after command options.
- A unit test walks the clap command tree and fails on any argument with empty help.

## Context

`crates/atelier-cli/src/main.rs` (derive definitions). atelier-kaor fixed only the `issue` options.
