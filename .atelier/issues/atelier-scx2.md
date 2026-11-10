---
created_at: "2026-11-10T01:14:38.531271779+00:00"
id: "atelier-scx2"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Freeze the baseline's inputs, so the numbers can be reproduced and the cutover need not wait for the report"
updated_at: "2026-11-10T01:29:37.229558749+00:00"
---

## Goal

Copy every input the baseline reads, once, so that later runs give the same numbers, nothing reads T3's live database again, and the cutover can go ahead before the report is written. Transcripts are disappearing: Claude's survive for 13% of threads and Codex keeps a few days.

## Done when

- [ ] `~/.local/share/atelier-baseline/2026-11/` holds a copy of `~/.t3/userdata/statev2.sqlite` made with SQLite's online backup (`sqlite3 <db> ".backup <copy>"`), copies of the Claude transcript directories for atelier, astromech, demiurge, life-on-rails and cosplay-studio, including their T3 worktree directories, and a copy of every Codex session dated 2026-10-14 or later.
- [ ] `docs/architecture/quality/scorecard-baseline/manifest.json` is committed and gives the cutoff time, each of the five repositories' base commit at the cutoff, and each copied file's path under the snapshot, size and SHA-256. It holds no message text.
- [ ] `PRAGMA integrity_check` on the copy returns `ok`, and the Report records it.
- [ ] Nothing under the snapshot is committed, and the snapshot directory is readable only by its owner.

## Scope

Copying and the manifest. No measurement. The backup runs once; do not query the live database for anything else.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`, Design. T3's database is 3.9 GB and in live use by the server Sumeet's phone depends on; the online backup is the safe way to copy it, and the disk has 390 GB free. Claude transcripts are under `~/.claude/projects/`, one directory per checkout, named for the path with `/` and `.` as `-`; Codex sessions are under `~/.codex/sessions/`, by date. The snapshot holds private messages: keep it out of the repository and send nothing from it to any outside service.
