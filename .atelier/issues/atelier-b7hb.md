---
created_at: "2026-10-15T21:00:45.353749565+00:00"
id: "atelier-b7hb"
issue_type: "feature"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Represent size, unverified assumptions, won't-do decisions and already-built work"
updated_at: "2026-10-15T21:00:45.353749565+00:00"
---

## Goal

Backlogs commonly carry a size estimate, a marker that an assumption hasn't been checked, a won't-do list with reasons, and a note of what already exists. Atelier has none of these: sizes became `size-*` labels, assumptions became prose (only epics have Unknowns), won't-do became a table in a strategy file, and built work went into epic Scope prose, because a done record needs a done claim and evidence. Decide which deserve first-class support.

## Done when

A decision record lists each of the four with its chosen representation, and any new field or record type has `check` and display support.

## Context

astromech's backlog had S/M/L/XL sizes, ⚠ unverified markers, a nine-row can't/won't table, and ✅ built rows (E1, E2, N1, V0).

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.
