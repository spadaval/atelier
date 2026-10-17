---
created_at: "2026-10-17T01:10:57.989574856+00:00"
id: "atelier-qrwr"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
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
status: "todo"
title: "Make not-found and invalid-value errors consistent, with short IDs everywhere"
updated_at: "2026-10-17T06:27:40.095358227+00:00"
---

## Goal

- **Not-found wording.** It has five forms: "Issue zzzz was not found", "issue zzzz not found" (merge), "landing issue not found" (`land zzzz`, no ID), "evidence record zzzz not found", "review zzzz not found". None gives a recovery or a near match. For an unknown short ID like `1w8` (prefix of 1w8c) or `7` (prefix of four IDs), say what might be meant.
- **Short IDs.** They resolve only for issues (`record_index.rs:931`, exact suffix and case-sensitive, so `1W8C` fails). `evidence show waj0`, `review show 0c5b` and `review list 37uq` fail or print nothing. `review list zzzz` exits 0 with no output. The wrong-kind hint ("X is a evidence record, not an issue record. Use ...", lint.rs:369, which also says "a evidence") only works for full IDs.
- **Invalid values.** `issue list --status bogus` → "Use an exact workflow status or `all`." and `--category open` → "Use a category from the workflow policy." list no values, while `--priority` and `--issue-type` do. Agents tried `-s open`.
- **Flag conflicts.** `work mission X --ready --blocked` → runtime "choose only one mission work filter" with exit 1, rather than clap's exit 2.
- **Wrong-kind errors.** `work mission sdb0` → "is issue_type 'feature'; work mission requires a mission", and `work parent 1w8c` → "(use work mission for missions ...)". Neither names the command with the ID (`atelier work mission atelier-1w8c`).

## Done when

- One message, `<kind> <ref> not found`, plus up to three near matches or `atelier issue list`.
- Every ID-taking command (issues, evidence, reviews) resolves case-insensitive short IDs and keeps the kind hint. Unknown IDs exit 1.
- Status and category errors list the workflow's values.
- Mutually exclusive flags use clap `conflicts_with`.
- Wrong-kind errors name the related mission or parent command.
- Tests for each case.

## Context

`check atelier-zzzz` claims "Tracker record files are invalid" while `check zzzz` says not found. That is filed as its own check item in this mission; keep the wording here consistent with it. main.rs:497-520 and issue.rs:28-51 hold two copies of the wrong-kind helper.
