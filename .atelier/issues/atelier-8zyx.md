---
created_at: "2026-08-05T18:47:09.160608343+00:00"
id: "atelier-8zyx"
issue_type: "task"
labels:
- "validation-finding"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Repair mission authoring and contract residue"
updated_at: "2026-08-05T18:47:55.909804593+00:00"
---

## Description

Repair the independent validation findings around mission `--body` authoring, stale or contradictory product guidance, empty extended-test inventory guidance, and committed evidence whitespace. Keep mission authoring simple and produce structurally valid records for every successful command.

## Outcome

- Mission `--body` either produces valid Description/Outcome sections or is removed with current help/docs/tests aligned.
- Supersession, ready transitions, Markdown section contracts, and ignored-test guidance agree with implemented behavior.
- The branch-range whitespace check passes and fresh authoring regressions cover the corrected path.

## Evidence

- Fresh mission creation and `atelier check` succeed together.
- Docs/help residue, ignored-test inventory, and `git diff --check mission/atelier-5kej...HEAD` are reviewed explicitly.
