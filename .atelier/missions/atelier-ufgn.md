---
created_at: "2026-06-11T20:09:59.016928901+00:00"
id: "atelier-ufgn"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-24sg"
    type: "advances"
  - kind: "issue"
    id: "atelier-29oi"
    type: "advances"
  - kind: "issue"
    id: "atelier-3tkt"
    type: "advances"
  - kind: "issue"
    id: "atelier-7n3w"
    type: "advances"
  - kind: "issue"
    id: "atelier-pjai"
    type: "advances"
  - kind: "issue"
    id: "atelier-wznt"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Markdown-first single-tree storage overhaul"
updated_at: "2026-06-12T00:31:23.405885419+00:00"
---

## Intent

Move Atelier to a single committed .atelier/ tree where canonical Markdown records are the source of truth and runtime/cache state is explicitly ignored. This is a planning mission only: implementation starts after the linked contract, migration, validation, and sequencing issues are accepted.

Desired outcome:
- .atelier/issues, .atelier/missions, .atelier/plans, .atelier/evidence, activity records, and related committed records are canonical Markdown.
- .atelier/cache or .atelier/runtime contains ignored local projection/runtime files such as state.db, source metadata, diagnostics, identity, locks, and UI caches.
- SQLite is a rebuildable ProjectionIndex for metadata, search, and graph/query acceleration, never the durable copy of full record bodies.
- All commands share one freshness policy and fail clearly on invalid canonical Markdown instead of reading stale projection state.
- Direct Markdown edits are supported through documented contracts and validated by atelier lint.
- Migration, backcompat, merge-conflict, notes/activity, and no-SQLite recovery behavior are proven with tests.

## Constraints

- Do not implement during planning; keep existing .atelier-state compatibility read-only until an explicit migration slice lands.
- Canonical Markdown and runtime/cache concerns must remain separate in commands, docs, and tests.

## Risks

- Persistence migration can corrupt durable tracker state if path resolution, ignore policy, and projection refresh are not centrally owned.

## Validation

- Mission closeout requires workflow validation, lint, doctor, migration fixtures, direct-edit recovery tests, cache deletion recovery tests, cargo fmt -- --check, and cargo nextest run.
