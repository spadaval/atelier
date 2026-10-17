---
created_at: "2026-10-17T01:10:57.986805376+00:00"
id: "atelier-8zxh"
issue_type: "bug"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Stop work ready silently dropping rows past 20"
updated_at: "2026-10-17T01:10:57.986805376+00:00"
---

## Goal

`IssueListPanel::new` defaults to `limit: 20` (crates/atelier-cli/src/human_output.rs:333-339) and `work::list()` never overrides it. With 22 ready items the fixture printed `... 2 more ready work items omitted` with no flag or command to see them, and the rows cut were the lowest-priority ones, including atelier-ubti, which unblocks a 4-deep chain. Agents went looking: astromech-atelier ran `atelier work ready --all … || atelier work ready -h` and `atelier work ready --json` (4 calls), then `atelier work all | grep '\[todo\]' | awk …`.

## Done when

- Either `work ready`, `work blocked`, `work human` and `work all` print every row, or they accept `--all` and the omission line reads `… 2 more: atelier work ready --all`.
- A test with 22 ready items shows all 22 rows (or the hint and all 22 under `--all`).
