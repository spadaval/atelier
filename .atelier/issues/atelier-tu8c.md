---
created_at: "2026-11-09T00:45:57.947764486+00:00"
id: "atelier-tu8c"
issue_type: "task"
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
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record ADR 0030: Atelier may have a site, a session database and a T3 dependency"
updated_at: "2026-11-09T00:45:57.947764486+00:00"
---

## Goal

Record decision atelier-7bs1 as ADR 0030 and rewrite `PRODUCT_INTENT.md`'s Boundaries to match, so later items build a server, a session database and T3 coupling without contradicting the product intent.

## Done when

- [ ] `docs/adr/0030-*.md` states the decision, what stays invariant (records in Git are the only source of truth for work; `merge`, `land`, `check` and `doctor` read only Git), what the session database may and may not hold, that the site is read-only until a later decision, and that attribution goes through the CLI.
- [ ] `PRODUCT_INTENT.md` no longer rules out a human UI, a long-running read-only server, a host-wide session database or a T3 Code dependency, and its "no SQLite cache" line says it means a cache of records.
- [ ] `docs/index.md` and `docs/adr/` index entries link ADR 0030, and `git grep -n -i "second UI\|not an agent runner, daemon"` finds no statement that contradicts it.

## Scope

Docs only. Do not design the server or schema here beyond what the ADR must say.

## Context

The decision record and `docs/strategies/fleet-dashboard.md` hold the reasoning. ADR 0019 and the rebuild strategy explain why the Boundaries were written.
