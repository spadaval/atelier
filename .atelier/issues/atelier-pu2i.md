---
created_at: "2026-10-17T01:10:57.964569101+00:00"
id: "atelier-pu2i"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show open work before done work in issue list"
updated_at: "2026-10-17T01:10:57.964569101+00:00"
---

## Goal

`atelier issue list` sorts by ID and stops at 50 rows (`crates/atelier-app/src/issue_inventory.rs:70-89`; "neutral ID ordering" in `docs/product/issue-inventory-and-mission-overview.md`). On master (86 records, 67 done) the default view shows 39 done rows and only 11 of the 19 open records. atelier-jjvc, kops, majk, mp6v, r6bt, sdb0, stsa and w0su are hidden behind `Showing 50 of 86 issues`. Agents work around it with `--limit 200 | grep -v ' done '` (4 runs), `--status todo --limit 100 | grep` (9+ runs), and `-s open`, which errors. The page is also padded with generic lines: an `Issue Inventory` title, `Narrow the metadata filters or raise --limit.`, and a `Drill Down / Inspect a record: atelier issue show <issue-id>` block on every run (`crates/atelier-cli/src/human_output.rs:429-452`).

## Done when

- With no filters, non-done records are listed before done ones (or done ones are left out), and the footer counts what is hidden, e.g. `19 open shown · 67 done hidden: atelier issue list --category done`.
- When rows are cut, the truncation line gives the exact next command, e.g. `36 more: atelier issue list --limit 86`.
- The `Issue Inventory` title, the "metadata filters" sentence and the `Drill Down` block are gone.
- A CLI test with 55 records, 50 of them done, shows every open one by default. Rendering tests are updated. `issue-inventory-and-mission-overview.md` states the new order.

## Scope

Inventory ordering, default filter and footer, their tests, and the doc. `--quiet` order stays deterministic.

## Context

ADR 0022 (dense Markdown; piping means a view is missing). Related: atelier-fpxc (text search), atelier-deyy (dense rows).
