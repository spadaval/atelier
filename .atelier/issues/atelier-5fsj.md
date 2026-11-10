---
created_at: "2026-11-10T00:56:05.416973437+00:00"
id: "atelier-5fsj"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-5fsj
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Script the scorecard's counted measures, so the baseline can be rerun after the cutover"
updated_at: "2026-11-10T14:15:28.429508957+00:00"
---

## Goal

Write the scripts that compute the scorecard's counted measures for a repository and a window, and run them for the baseline. Mission 4 reruns them after the cutover, so they are kept in the repository and reviewed like code.

## Done when

- [ ] `scripts/scorecard/README.md` says how to run each script, which inputs it reads, and which T3 database schema it was written against.
- [ ] Every script reads its inputs from a snapshot directory given as an argument, and the manifest's commits for Git. No script opens `~/.t3/userdata/` or the live transcript directories; a test asserts that the default paths are not used.
- [ ] Running them on the baseline snapshot for atelier, astromech, demiurge, life-on-rails and cosplay-studio writes `docs/architecture/quality/scorecard-baseline/counted.json` and `docs/architecture/quality/scorecard-baseline/counted.md`, giving per repository: process share, confusion, tokens per merged item, calls before first edit, rework, the record-only share of commits, and the line count. Each token number has its coverage beside it, and rework says which commits were old enough to count.
- [ ] `python3 -m unittest discover scripts/scorecard` passes. Its tests build a small SQLite file and a scratch Git repository and check each measure's arithmetic, so a changed definition fails a test.
- [ ] A script given a T3 database whose schema it does not know exits with a message naming the schema it expected.
- [ ] A fresh agent given only the README reran the scripts on the same snapshot and got a byte-identical `counted.json`; the Report records who and when.

## Scope

Python with the standard library only. Nothing changes in the `atelier` binary. The classified measures and the defect count are other items.

## Context

Each measure's definition is in `docs/strategies/redesign-1-baseline.md`, Design; do not change a definition, and escalate if one cannot be computed as written. The snapshot is at `~/.local/share/atelier-baseline/2026-11/`, described by `docs/architecture/quality/scorecard-baseline/manifest.json`. Starting material, not to be committed as it is: `/var/tmp/atelier-assessment/transcripts/` and `/var/tmp/atelier-assessment/history/`. Tie a thread to an item by the branch it worked on and the item IDs in its title and first message.
