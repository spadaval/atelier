---
created_at: "2026-10-15T22:08:59.029206534+00:00"
id: "atelier-ox63"
issue_type: "feature"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3mjv"
    type: "related"
  - kind: "issue"
    id: "atelier-q8ok"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Give decisions a record shape"
updated_at: "2026-10-15T22:08:59.029206534+00:00"
---

## Goal

Design questions can be filed (atelier-28mz is a task titled "Decide …"), but nothing marks them as decisions. They have no Options or Decision section, and they show up in `work ready` like code work even though most need the human and no branch.

## Done when

A decision type (or a typed section on any issue) with Question, Options and Decision; closing requires the Decision to be filled; `work ready` and the needs-human marking (atelier-q8ok) treat decisions as the human's by default; they can close on the base branch under the planning-change rule (atelier-3mjv).

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Sumeet: there should be a way to file issues for design questions. Today they're filed as tasks.
