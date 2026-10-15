---
created_at: "2026-10-15T22:08:58.988204415+00:00"
id: "atelier-ymmb"
issue_type: "bug"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9vq3"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "List only startable work in work ready"
updated_at: "2026-10-15T22:08:58.988204415+00:00"
---

## Goal

`atelier work ready` lists missions and epics next to tasks. Missions own no branch, and epics are worked through their children, so neither is something to start.

## Done when

`work ready` lists items that `atelier start` would accept; missions and parents appear only as context (for example a heading or a flag such as `--all`). A test covers a mission with a ready epic and child.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Seen in astromech: both missions and several epics appeared in the ready list. May be shaped by atelier-9vq3.
