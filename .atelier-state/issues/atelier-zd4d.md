---
acceptance: []
blocks: []
created_at: "2026-06-10T02:57:51.275067274+00:00"
depends_on: []
evidence_required: []
id: "atelier-zd4d"
issue_type: "task"
labels:
- "architecture"
- "epic"
- "markdown"
- "projection"
- "sqlite"
- "storage"
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Epic: Markdown-first record store and projection index architecture"
updated_at: "2026-06-10T02:57:51.275067274+00:00"
---

Rework Atelier toward a Markdown-first persistence architecture where canonical project state lives in record files and SQLite is a rebuildable projection/index plus clearly separated local runtime state, not a fully equivalent live copy of the canonical records.

## Problem

The current SQLite-centered implementation makes it tempting to treat the database as persistent domain storage and export Markdown as a projection. That creates split-brain risk, extra export machinery, and unclear ownership of canonical facts. A pure Markdown system is conceptually simpler but needs fast indexing for global queries, graph traversal, search, validation, and Mission Control-style views.

## Direction

Adopt a write-to-Markdown, refresh-projection model:

- RecordStore owns canonical Markdown reads and writes.
- ProjectionIndex owns rebuildable SQLite indexes for global queries and validation.
- RuntimeState owns local-only live data such as sessions, locks, timers, usage, and UI/cache state.
- Known-ID mutations update Markdown directly, then reindex affected records.
- Query commands use the projection after cheap freshness checks and targeted reindex/rebuild.
- No successful canonical mutation should depend on SQLite export to become durable.

## Scope

- Define the architectural boundary between RecordStore, ProjectionIndex, and RuntimeState.
- Identify which existing commands should become Markdown-direct writes and which should remain projection-backed queries.
- Introduce freshness/reindex semantics based on source metadata and content hashes, with mtime/size only as hints.
- Preserve SQLite where it provides real value: compound filters, reverse lookups, graph traversal, FTS/search, validation, and local runtime state.
- Avoid introducing a persistent daemon until an interactive workflow proves it is needed.

## Acceptance

The repo has a documented architecture and executable migration plan for Markdown-first writes plus rebuildable projection indexing; tracker children cover RecordStore APIs, projection refresh/freshness, command migration slices, runtime-state separation, validation, and docs. The plan explicitly rejects fully equivalent SQLite/Markdown live-state sync as the destination and classifies any deferred compatibility or daemon work.
