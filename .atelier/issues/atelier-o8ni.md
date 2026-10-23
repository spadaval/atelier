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
closed_at: "2026-10-23T16:10:20.657509457+00:00"
status: "done"
title: "Make evidence show brief: one transcript, no empty fields, freshness"
updated_at: "2026-10-23T16:10:20.657509457+00:00"
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
- 2026-10-23 [decision] Brief show's title line is '<id> evidence recorded' followed by the receipt line's facts, with no underline; the title moves to a Summary: line when it differs from the command
  Reason: The regroom note keeps 'recorded' in the title line (human-cli-output.md item order <id> <type> <status>); a long summary no longer sets a 400-character underline.
  Rejected: Dropping 'evidence recorded' from the brief title

## Close Reason

`evidence show <id>` (short IDs work) is now brief: `<id> evidence recorded ⚠ fail test for issue/<id> @<pin> · <captured> · <standing>`, then `Summary:` when it differs from the command, `Command: … (exit n)`, the optional fields that are set, and one output excerpt (a pass's test-count line, or the last 20 lines of the stream that kept text). There is no Status, Created, Updated or `(none)`, and the stream appears once. `--full` prints every field, including freshness, and then the stored record once.

### Deviations
The brief view has no underlined title. The freshness word for a blocking fail is `⚠ blocks completion` rather than `stale|fresh`.

### Decisions
See the decision note on atelier-o8ni about the title line.

### Surprises
None.

### Open items
None.
