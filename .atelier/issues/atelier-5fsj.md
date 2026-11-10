---
created_at: "2026-11-10T00:56:05.416973437+00:00"
id: "atelier-5fsj"
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
title: "Script the scorecard's counted measures, so the baseline can be rerun after the cutover"
updated_at: "2026-11-10T00:56:05.416973437+00:00"
---

## Goal

Write the scripts that compute the scorecard's counted measures for a repository and a window, and run them for the baseline. Mission 4 reruns them after the cutover, so they are kept in the repository and reviewed like code.

## Done when

- [ ] `scripts/scorecard/README.md` says how to run each script, which stores it reads, and which T3 database schema it was written against.
- [ ] Running them for atelier, astromech, demiurge, life-on-rails and cosplay-studio over the baseline window writes `docs/architecture/quality/scorecard-baseline/counted.json` and `docs/architecture/quality/scorecard-baseline/counted.md`, giving per repository: process share, confusion, tokens per merged item, calls before first edit, rework, the record-only share of commits, and the line count. Each token number has its coverage beside it.
- [ ] `python3 -m unittest discover scripts/scorecard` passes. Its tests build a small SQLite file and a scratch Git repository and check each measure's arithmetic, so a changed definition fails a test.
- [ ] Every connection to T3's database is opened read-only, and a test asserts it. Every query is bounded by an indexed column or a LIMIT.
- [ ] A script given a T3 database whose schema it does not know exits with a message naming the schema it expected.
- [ ] A fresh agent given only the README reran the scripts on the same window and got the same `counted.json`; the Report records who and when.

## Scope

Python with the standard library only. Nothing changes in the `atelier` binary. The classified measures and the defect count are other items.

## Context

Each measure's definition is in `docs/strategies/redesign-1-baseline.md`, Design; do not change a definition, and escalate if one cannot be computed as written. Starting material, not to be committed as it is: `/var/tmp/atelier-assessment/transcripts/` and `/var/tmp/atelier-assessment/history/`. T3's database is `~/.t3/userdata/statev2.sqlite`, 4 GB and in live use. Tie a thread to an item by the branch it worked on and the item IDs in its title and first message.
