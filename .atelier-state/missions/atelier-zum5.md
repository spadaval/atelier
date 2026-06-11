---
created_at: "2026-06-10T20:16:22.904894403+00:00"
id: "atelier-zum5"
data: "{\"constraints\":[\"Canonical project state lives in deterministic Markdown records under .atelier-state/.\",\"SQLite remains a rebuildable ProjectionIndex and local RuntimeState, not the durable owner of project facts.\",\"Migration slices must preserve rebuild, export --check, lint, doctor, and the agent-facing issue workflow unless temporary breakage is explicitly tracked.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Split-brain behavior if SQLite and Markdown continue to act as competing durable sources.\",\"Stale projection answers could mislead orchestration, validation, or closeout decisions.\"],\"validation\":[\"Known-ID issue mutations become Markdown-first writes followed by projection refresh or stale marking.\",\"Projection freshness detects changed, missing, or unindexed canonical sources before query commands answer.\",\"RuntimeState health is reported separately from canonical projection rebuild readiness.\"],\"work\":[]}"
links:
- target_id: "atelier-a4ps"
  target_kind: "issue"
  type: "blocked_by"
- target_id: "atelier-zd4d"
  target_kind: "issue"
  type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Markdown-first durable state architecture cutover"
updated_at: "2026-06-10T23:57:40.556245283+00:00"
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

## Execution Graph

- Executable epic/work package: atelier-zd4d.

## Acceptance

The repo has a documented architecture and executable migration plan for Markdown-first writes plus rebuildable projection indexing; tracker children cover RecordStore APIs, projection refresh/freshness, command migration slices, runtime-state separation, validation, and docs. The plan explicitly rejects fully equivalent SQLite/Markdown live-state sync as the destination and classifies any deferred compatibility or daemon work.
