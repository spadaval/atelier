---
created_at: "2026-11-11T02:17:26.543855657+00:00"
id: "atelier-e05n"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uwxy"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show the agents working on a mission and its items, from T3 Code's MCP server"
updated_at: "2026-11-11T02:17:26.543855657+00:00"
---

## Goal

Preliminary. The Desk's server becomes a client of T3 Code's MCP server with a read-only token, and mission, item and inbox pages show each thread's model, state and link, tied to items by the thread IDs in the records.

## Done when

- [ ] Provisional: with T3 Code running, a mission page lists the threads recorded on its items with live state and a working link; a thread waiting on the human shows in the inbox; with T3 Code stopped, the pages render and say threads are unavailable. Nothing reads T3 Code's own database.
