---
created_at: "2026-06-11T15:53:41.142956011+00:00"
id: "atelier-rr6y"
issue_type: "epic"
labels:
- "markdown"
- "projection"
- "sqlite"
- "storage"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-64w5"
  - kind: "issue"
    id: "atelier-ncog"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Finish Markdown-first writes and transparent projection rebuild"
updated_at: "2026-06-11T15:57:34.759490885+00:00"
---

## Description

Finish the Markdown-first persistence transition so canonical durable records are written through RecordStore first and SQLite is only a transparent rebuildable projection/cache.

## Outcome

- Command audit identifies every durable mutation path and classifies it as Markdown-first, compatibility SQLite-first, or runtime-only.

- Remaining non-runtime durable mutations either write Markdown first or have child issues documenting the migration boundary.

- Projection-backed read commands automatically rebuild stale SQLite from `.atelier-state/` when validation passes and the command contract is safe to repair.

- Stale projection tests cover changed, missing, and unindexed Markdown sources.

- Invalid canonical Markdown still produces actionable errors and never silently rebuilds from stale or corrupt data.

- Closeout validation covers fresh checkout rebuild, manual Markdown edit followed by query, concurrent mutation/query behavior, export freshness, lint, doctor, and nextest/cargo test coverage as appropriate.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Desired State

1. Most durable command mutations write directly to Markdown records before touching SQLite.
2. Projection-backed commands automatically and transparently rebuild SQLite when the projection is stale, instead of requiring operators to run `atelier rebuild` for ordinary safe query paths.

### Scope

- Move remaining durable mutation paths for missions, plans, evidence, links, labels, dependencies, and issue lifecycle gaps toward RecordStore-owned Markdown writes followed by projection refresh.
- Define the narrow cases where SQLite-first compatibility remains temporarily acceptable, and track follow-up slices explicitly.
- Replace fail-closed stale projection behavior with safe automatic rebuild for read-only query paths when `.atelier-state/` validates.
- Preserve explicit failure for invalid Markdown, conflicting records, unsafe concurrent writes, or runtime DB repair failures.
- Keep runtime-only state under `.atelier/` separate from canonical projection rebuilds.
- Ensure `atelier export --check`, `atelier rebuild`, `atelier lint`, `atelier doctor`, issue workflows, mission workflows, and Mission Control inputs remain correct.
