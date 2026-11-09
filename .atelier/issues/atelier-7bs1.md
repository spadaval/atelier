---
created_at: "2026-11-09T00:45:57.358224775+00:00"
id: "atelier-7bs1"
issue_type: "decision"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T00:46:17.551421462+00:00"
status: "done"
title: "Let Atelier have a human UI, a host-wide session database and depend on T3 Code"
updated_at: "2026-11-09T00:46:17.551421462+00:00"
---

## Question

May Atelier have a human UI, a long-running server, a host-wide database and a dependency on T3 Code? `PRODUCT_INTENT.md`'s Boundaries rule out a second UI, a daemon and a SQLite cache, and keep Atelier independent of any harness.

## Options

- Keep the Boundaries. The human keeps reaching project state only through agents, and documents such as HTML debriefs stay inside whichever checkout generated them.
- Static pages only: `atelier site` regenerates HTML after `land` and `tailscale serve` publishes the folder. No daemon, but no live thread status and no session database.
- Allow a read-only server, a host-wide session database and T3 coupling, with records in Git as the only source of truth for work and the guards reading nothing else.

## Decision

Allow them. The Boundaries were a reaction to the machinery the rebuild removed (a workflow engine, a review provider, a SQLite cache of records, a Mission Control TUI), not a judgment that humans should not act on the backlog. What stays: tracker records are Markdown in Git and the only source of truth for work; `merge`, `land`, `check` and `doctor` read only Git. The session database holds only what Git never held (which thread, harness, model and role ran each command), so it is the only copy of that data, not a cache. T3 Code, Atelier and Agent Factory are one stack on this host, so tight coupling to T3 is fine; forking T3 is a last resort. Attribution is recorded by the CLI, which inherits each session's environment, rather than by an MCP server. The site is read-only in its first mission; write actions need a later decision. Decided by Sumeet on 2026-11-08 in a design discussion; ADR 0030 records it (mission fleet-dashboard's first item).

## Report

Allowed: a read-only site and server, a host-wide session database holding only what Git never held, and coupling to T3 Code; records in Git stay the only source of truth for work and the guards read only Git. Decided by Sumeet on 2026-11-08; ADR 0030 records it (atelier-tu8c).
