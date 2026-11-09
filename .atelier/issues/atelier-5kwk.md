---
created_at: "2026-11-09T00:45:59.392192270+00:00"
id: "atelier-5kwk"
issue_type: "task"
labels:
- "fleet-dashboard"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7yy9"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remind an agent committing in a T3 thread on an unowned branch to run atelier start"
updated_at: "2026-11-09T00:45:59.392192270+00:00"
---

## Goal

Nudge agents to record their work: when an agent in a T3 thread commits code on a branch with no started item, the harness hook reminds it to run `atelier start`, so its tokens and commits are attributed.

## Done when

- [ ] Under `atelier setup claude` hooks (and the Codex equivalent if one exists), a commit in a T3 thread on a branch no item owns prints one reminder naming `atelier start <id>`, and a commit on an owned branch prints nothing. Tests cover both.
- [ ] The reminder never blocks the commit.

## Context

A nudge, not a gate: agents are trusted, and this only improves attribution.
