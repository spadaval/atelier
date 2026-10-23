---
created_at: "2026-10-23T18:02:22.274310522+00:00"
id: "atelier-wir7"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1o56"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fix three small misleading lines found in rc8j acceptance"
updated_at: "2026-10-23T18:02:22.274310522+00:00"
---

## Goal

When an owner's start transition is recorded but not committed, `merge --check` prints `merge_owner_unstarted … Next: atelier start <id>`; rerunning start does not clear it, the records-commit fix printed beside it does. Print the records-commit fix (or fold the line into the uncommitted-records line) in that case. Separately, `merge <id> --sync` ends `merge: passed` when it merged nothing although `merge --check` would refuse; say what sync did and leave the gate verdict to `--check` (e.g. `merge: synced`). And `review show` on an author-identity approval says only `not accepted on <base>`, not that the identity matches a Git author.

## Done when

Tests pin each of the three lines.

## Context

Found by acceptance validation atelier-1o56.
