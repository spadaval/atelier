---
created_at: "2026-11-09T00:45:59.392192270+00:00"
id: "atelier-5kwk"
issue_type: "task"
labels:
- "agent-attribution"
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
    id: "atelier-bnek"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remind an agent committing in a T3 thread on an unowned branch to run atelier start"
updated_at: "2026-11-09T01:04:27.532061591+00:00"
---

## Goal

Nudge agents to record their work: when an agent in a T3 thread commits code on a branch no item owns, the harness hook reminds it that work on an item should start with `atelier start`, while saying light-path work needs nothing, so attribution improves without implying every change needs an issue.

## Done when

- [ ] Under `atelier setup claude` hooks (and the Codex equivalent if one exists), a commit in a T3 thread on a branch no item owns, in a thread whose session has recorded no item, prints one reminder that names `atelier start <id>` for item work and says a light-path change needs no item. A commit on an owned branch, or in a thread that has recorded an item, prints nothing. Tests cover all three.
- [ ] The reminder never blocks the commit and appears at most once per thread and branch.

## Context

A nudge, not a gate: agents are trusted, and this only improves attribution. The light path (`PRODUCT_INTENT.md`, `docs/product/reviews-and-merge.md`) lets releases, docs and mockups merge with no issue (planning review finding atelier-726p).
