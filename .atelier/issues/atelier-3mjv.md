---
created_at: "2026-10-15T22:08:58.949610245+00:00"
id: "atelier-3mjv"
issue_type: "task"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Decide how planning-only changes reach the base branch"
updated_at: "2026-10-15T22:08:58.949610245+00:00"
---

## Goal

Records live on branches, so filing, grooming, re-prioritising and strategy edits are commits too. Nothing says how they land. In practice the atelier repo commits them straight to master (148f3f8c, 547c761a), which `atelier merge` can't gate and the docs don't sanction. Re-prioritising on a work branch instead conflicts with other branches.

## Done when

The docs name the path for record-only changes: either "record-only commits may land on the base branch directly" with a `check` that the commit touches only `.atelier/` and strategies, or a lightweight gated path (for example `atelier plan commit`). `land` on the base branch accepts it.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Related: atelier-40i6, and the two ways to start work in atelier-kaor.
