---
created_at: "2026-10-15T22:08:59.015438310+00:00"
id: "atelier-r7cd"
issue_type: "task"
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
title: "Decide what pruning keeps"
updated_at: "2026-10-15T22:08:59.015438310+00:00"
---

## Goal

Prune removes terminal records after 7 days. That's fine for routine work, but research, decisions and references attached to closed work then exist only in Git history, which weakens "records hold everything". It's also unverified what happens to live links that point at a pruned record.

## Done when

Sumeet decides what survives pruning (for example records with a Decision or research section, or anything linked from an open record, or a per-record keep flag), the rule is implemented, and `check` handles links to pruned records with a test.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Related: atelier-by8u (a place for research and references).
