---
created_at: "2026-10-17T01:10:57.975261389+00:00"
id: "atelier-o8ni"
issue_type: "task"
labels:
- "cli-output"
- "evidence"
priority: "P2"
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
title: "Make evidence show brief: one transcript, no empty fields, freshness"
updated_at: "2026-10-23T14:47:58.928636359+00:00"
---

## Goal

`atelier evidence show atelier-1qkb` prints the stderr excerpt twice: once in the Markdown body (`## Stderr`) and again under "Output Summary" (crates/atelier-cli/src/commands/evidence.rs:719). It also shows `Status: recorded` (the same on every receipt), `Producer/Path/URI: (none)`, Created and Updated seconds apart, a full 40-character commit, and a `====` underline as long as the title (about 400 characters for atelier-3onc). It doesn't say whether the receipt still proves its target. `evidence show 1qkb` fails with `evidence record 1qkb not found`.

## Done when

- The default view starts with `atelier-1qkb ⚠ fail test for issue/atelier-cafy @bc2dd311 · 2026-10-16 18:21 · stale|fresh`, then the command and exit status, then one output excerpt (tail first). Empty fields and Status/Created/Updated are left out.
- `--full` prints the stored body and every field.
- Short IDs resolve. A test pins both views.

## Context

This follows the brief-by-default and `--full` rule that atelier-7u4o sets for `issue show`.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: short IDs resolve, quiet show prints '<id> <result>'. Still: output twice (print_record_with_targets :771 then print_output_summary), (none) fields, Created/Updated, no freshness, no --full. Keep 'recorded' in the title line (human-cli-output.md:73).
