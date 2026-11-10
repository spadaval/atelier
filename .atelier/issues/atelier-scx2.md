---
created_at: "2026-11-10T01:14:38.531271779+00:00"
id: "atelier-scx2"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-scx2
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
status: "in_progress"
title: "Freeze the baseline's inputs, so the numbers can be reproduced and the cutover need not wait for the report"
updated_at: "2026-11-10T05:26:13.895593564+00:00"
---

## Goal

Copy every input the baseline reads, once, so that later runs give the same numbers, nothing reads T3's live database again, and the cutover can go ahead before the report is written. Transcripts are disappearing: Claude's survive for 13% of threads and Codex keeps a few days.

## Done when

- [x] `~/.local/share/atelier-baseline/2026-11/` holds a copy of `~/.t3/userdata/statev2.sqlite` made with SQLite's online backup (`sqlite3 <db> ".backup <copy>"`), copies of the Claude transcript directories for atelier, astromech, demiurge, life-on-rails and cosplay-studio, including their T3 worktree directories, and a copy of every Codex session dated 2026-10-14 or later.
- [x] `docs/architecture/quality/scorecard-baseline/manifest.json` is committed and gives the cutoff time, each of the five repositories' base commit at the cutoff, and each copied file's path under the snapshot, size and SHA-256. It holds no message text.
- [x] `PRAGMA integrity_check` on the copy returns `ok`, and the Report records it.
- [x] Nothing under the snapshot is committed, and the snapshot directory is readable only by its owner.

## Scope

Copying and the manifest. No measurement. The backup runs once; do not query the live database for anything else.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`, Design. T3's database is 3.9 GB and in live use by the server Sumeet's phone depends on; the online backup is the safe way to copy it, and the disk has 390 GB free. Claude transcripts are under `~/.claude/projects/`, one directory per checkout, named for the path with `/` and `.` as `-`; Codex sessions are under `~/.codex/sessions/`, by date. The snapshot holds private messages: keep it out of the repository and send nothing from it to any outside service.

## Notes

- 2026-11-10 [note] Handoff: approved by gpt-6.1-sol-scx2-r1 (atelier-sqo2) at 03732309 after one round with no findings. See the Report. Merge waits only on CI (atelier merge atelier-scx2 --push --wait), which the orchestrator runs.

## Report

All four Done when lines hold. The snapshot at `~/.local/share/atelier-baseline/2026-11/` was made once, at cutoff 2026-11-10T05:14:58Z, by `scripts/scorecard/freeze.py copy`. It holds 4,197 files and 7.54 GB: T3's database (1 file, 4.13 GB), 4,053 Claude transcript files from 31 directories (3.23 GB) and 143 Codex sessions (0.18 GB). `freeze.py verify` reports that every file matches the manifest's size and SHA-256, that every manifest entry exists, and that every mode is owner-only (directories 700, files 600, the database 400). Fed a tampered manifest and a loosely-moded scratch tree, it reports each changed, missing and unlisted file and each bad mode. `PRAGMA integrity_check` on the copy returns `ok`. `git ls-files` lists nothing from the snapshot.

The live database was opened once, read-only (`mode=ro`), by the sqlite3 CLI's `.backup`, at `nice -n 19 ionice -c3`. The backup took 22 seconds and succeeded on its first attempt. The copy's `user_version` is 0 and its schema fingerprint is in the manifest.

### Decisions

- The backup runs inside one read transaction (`BEGIN; SELECT count(*) FROM sqlite_schema; .backup ...; COMMIT`). The CLI copies 100 pages per step, and a write by another process between steps restarts it. The 4 GB database is written every few seconds, so a plain backup might never finish. The `SELECT` reads only the schema table, to start the transaction. A test on a scratch WAL database with a concurrent writer gave a consistent copy and `integrity_check` `ok`.
- The copy is switched to `journal_mode=DELETE` before hashing. A WAL-mode copy grows `-wal` and `-shm` files whenever a reader opens it, even read-only, and those files would break the manifest.
- Each base commit is the default branch's local tip. If the remote-tracking branch is strictly ahead, it is the remote-tracking tip instead, and both are recorded. Only astromech differs this way: `origin/main` is one commit ahead of local `main`, so its base is `ae4d18ba`. Local demiurge and cosplay-studio are ahead of their stale remote-tracking refs, so their local tips are used. Nothing was fetched.
- A Claude directory matches when its name contains the encoded path of a checkout of one of the five repositories, followed by `-` or the end of the name. The checkouts are each repository's `git worktree list`, every clone under `~/projects` with the same root commit, and T3's worktree root `~/.t3/worktrees/<repository>`, which catches removed worktrees. That gave 31 directories, listed with their repository in the manifest: 12 atelier, 3 astromech, 2 demiurge (including `worldcrafter`), 9 life-on-rails and 5 cosplay-studio. Three of the cosplay-studio directories are subagent working directories in a cosplay-studio thread's scratchpad. No `atelier-skill`, `atelier-spike-mc`, `astromech-orch`, `lor-*` or `srv/cosplay-studio` directory exists. The other 448 directories are eval sandboxes, temp dirs, T3 scratch threads and other repositories.

### Deviations

- `memory/` subdirectories (55 files of Claude's notes in four directories) were removed from the snapshot after the copy, and the script now skips them. They are not transcripts. Their file names are topic slugs, which a committed manifest should not carry.

### Surprises

- Codex keeps only 2026-11-08 to 2026-11-10, so "every session since 2026-10-14" is 143 files from those three days.
- Several transcript files belong to threads still running at the cutoff, including this one. Their copies run past the cutoff by up to 34 seconds; the manifest's `cutoff_means` says how to read them.

### Open items

- The snapshot is on one disk with no backup, as the strategy's Risks say.
