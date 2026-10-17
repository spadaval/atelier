---
created_at: "2026-10-17T01:10:57.991429157+00:00"
id: "atelier-i2qq"
issue_type: "task"
labels:
- "cli-output"
- "cross-cutting"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-e95f"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Write one item-line, verdict, confirmation and quiet contract for all views"
updated_at: "2026-10-17T01:10:57.991429157+00:00"
---

## Goal

The same item is written in at least seven shapes, and `[...]` means four different things. Neither an agent nor a human can learn one grammar. The human noticed `[todo]` sitting where a type would be expected, because headers bracket the type. Observed on master:

| Fact | Strings | Source |
|---|---|---|
| Row shape | `ready atelier-bw0b [todo] high - Title`; `atelier-1w8c  draft  high  Title`; `atelier-43m1  task  todo  low  Title`; `task/atelier-bw0b @ ae8e9ad9: atelier-bw0b [in_progress] Title — 1 active (committed branch records)`; `advances atelier-jjvc [backlog] low - Title`; `updated atelier-deyy [blocked] low - Title` | human_output.rs IssueListRow; work.rs:225; human_output.rs:385; git.rs branch_lines; issue.rs:229 |
| Bracket meaning | type in `atelier-1w8c [mission] draft - ...`; status in work rows; category in status Current Work `active atelier-43m1 [active]` two lines above `Owner: atelier-43m1 [in_progress]`; classification `[blocked]` for a status-todo item; record kind `atelier-waj0 [evidence] recorded` | issue.rs header; human_output.rs:324; status.rs:100; issue.rs:236; evidence.rs |
| Header repeats | header `draft` then `Status: draft`, `Type: mission`; status `Current work: none` then a `Current Work (none)` section | issue.rs, work.rs:402, status.rs:88/155 |
| Page title mid-output | status prints `Other local work branches` with a `=` underline and no blank line before it | status.rs:39 |
| Underline width | `"=".repeat(title.len())` counts bytes, so a 45-char title with é, ✓ or — gets a 52-char underline | human_output.rs:78, 643-648 |
| Commit hashes | `@ ae8e9ad9` (8) in status; full 40-hex in evidence show, review show, merge output | git.rs, evidence.rs, review.rs |
| Timestamps | `2026-10-16 20:31 -04:00` (issue show); `2026-10-16T23:51:24.389518170+00:00` (evidence show); `2026-10-16T18:47:25.090804002Z` (review show) | issue.rs:584; evidence.rs:676-697; review.rs:128 |

Other cross-cutting items (quiet, hints, footers, counts) need a written rule to point at.

## Done when

`docs/product/human-cli-output.md` states, and rendering tests enforce:
- **Item line:** `<id> <status> <priority> <title>` with an optional trailing `| <cue>`. The type appears only in views that mix types, as a plain word, never in brackets. Brackets are not used for state at all. Every view in the table renders it, and status's Current Work row agrees with its Owner row.
- **Hashes:** 8 characters in views. Full 40 only where the value must be passed back to a command (`--commit`, `--resolution-commit`).
- **Timestamps:** one display format, local `YYYY-MM-DD HH:MM ±hh:mm`. Records keep RFC 3339.
- **Headings:** one page title per command output; section underlines sized by display width.
- **Verdict line:** gating commands (`land`, `doctor`, `check`, `merge`, `merge --check`, `start`, `evidence record`) end with one line such as `land: passed` / `land: refused — 2 checks (land_records_uncommitted, ...)`, and keep a non-zero status on refusal. Agents pipe these through `tail` (land 132, check 146, evidence record 66 transcript runs), and one `| tail` masked a refused land before a push.
- **Mutation confirmation:** every write prints the resulting values it changed (for example `atelier-43m1 priority: high`).
- **Quiet tokens:** a table of each command's `-q` output (see the quiet item).

## Scope

The doc plus the shared renderers in `crates/atelier-cli/src/human_output.rs`, and the views listed in the table. Each command's own verdict and quiet behaviour is done in its own item. This item writes the rule and the shared row and heading helpers.

## Context

Extends atelier-deyy, which writes the first dense-output rule and the `work` view rows, so this is blocked by it. ADR 0022 Decision 3.

## Unknowns

- Design the item-line contract and the shared row and heading helpers so built-in views could later be expressed as declarative view definitions (filter, fields from a fixed vocabulary, shared sort, grouping) without changing their output. atelier-e95f, custom work views, depends on this. It is not a requirement here; it is a design constraint to keep open.
