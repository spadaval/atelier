---
created_at: "2026-10-17T01:10:57.978059331+00:00"
id: "atelier-3u2u"
issue_type: "bug"
labels:
- "cli-output"
- "review"
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
closed_at: "2026-10-23T14:35:51.473128189+00:00"
status: "done"
title: "Make review show a brief Markdown view with short IDs and linked events"
updated_at: "2026-10-23T14:35:51.473128189+00:00"
---

## Goal

`atelier review show` always prints pretty JSON (crates/atelier-cli/src/commands/review.rs:127-133). That breaks ADR 0022 Decision 3, and here JSON isn't an optional mode: it's the only output. The summary is printed twice (`header.title` and `summary`), along with `"labels": []`, empty `blocks`/`children`, and identical `created_at`/`updated_at`. Showing a finding doesn't show its dispositions, and showing a disposition doesn't show the finding text. Short IDs fail: `review show x16l` → `Error: review x16l not found`, and `review disposition 45rc …` fails the same way.

## Done when

- `review show atelier-x16l` prints a header line like `atelier-x16l approval for atelier-bw0b @e2562bd6 by Independent reviewer 3 · 2026-10-16 23:57 · ✓ current`, then the summary once.
- A finding also lists its disposition chain (`→ fix_now atelier-5an0 @cf6d13d9 "Changed to goodbye"`), and a disposition shows the first line of its finding.
- Short review IDs resolve in `review show` and `review disposition`.
- No `--json` flag is added. Tests cover a finding with two dispositions and short-ID lookup.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: short IDs resolve in show (review.rs:190) and disposition (:112); drop that bullet. Still prints pretty JSON (review.rs:204).
- 2026-10-23 [decision] review show judges an approval or exemption alone: on the work branch '✓ current' when its identity is independent and review_gate::current_on_branch holds, else '⚠ not current'; on the base approval_accepted's '✓ accepted on <base>' or '⚠ not accepted on <base>'.
  Reason: Same judges as review list's state block; the reason it is not current belongs to atelier-imlh.

## Close Reason

`atelier review show` is a brief view: a header `<id> <kind> on <owner> @<pin> by <identity> | <timestamp>` (an approval or exemption adds `| ✓ current` or its base equivalent), then a finding's disposition chain or a disposition's finding, `Fixed at`, `Follow-up` and `Supersedes` lines, then the text once. No JSON is printed and no `--json` was added.

### Deviations
The header uses ` | ` cues, as the item-line contract does, instead of the sketch's `·`. Short-ID resolution was already on master (regroom note).

### Decisions
See the decision note on this item (how a single approval's standing is judged).

### Surprises
None.

### Open items
None.
