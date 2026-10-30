---
created_at: "2026-10-30T06:52:23.609301229+00:00"
id: "atelier-6306"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remove one-way decision notes"
updated_at: "2026-10-30T06:52:23.609301229+00:00"
---

## Question

Should Atelier keep the `--one-way` mark on decision notes? Mission atelier-r6bt added it so costly-to-reverse choices lead `atelier work decisions` and the debrief. Agent Factory now says a choice that is costly to reverse is a `decision` record, not a note, and tells agents not to use `issue note --one-way` (spadaval/.agents, `skills/agent-factory/references/tracker-records.md`, Decision Notes). Atelier's help still advertises "one-way decision notes first".

## Options

- **Remove one-way notes.** Drop `--one-way` and the `[decision one-way]` tag; decision records lead the decision views. One way to record a costly choice. Costs a migration of the six existing one-way notes (atelier-fip8, ho7o, sfsq, all closed) and edits to the r6bt strategy and product docs.
- **Keep them.** No work, but Atelier keeps a second way to record a costly choice that the process tells agents not to use, and the two drift.

## Decision

Remove one-way decision notes. Agent Factory and Atelier ship as a pair (ADR 0020), and the human asks for one way to do a thing; a record is already the home for costly choices, with its own review and supersession. No other Atelier repository on this host uses the mark (astromech, astromech-atelier, astromech-orch, cosplay-studio and worldcrafter have none), so the six notes here are migrated to plain decision notes and no read compatibility is kept. Decided 2026-10-30 by the planning agent at the human's request to decide; it revises atelier-r6bt's strategy, whose lines on one-way decisions change in the removal work, atelier-w5hx.
