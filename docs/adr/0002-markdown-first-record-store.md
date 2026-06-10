# ADR 0002: Markdown-First Record Store

## Status

Accepted.

## Context

Atelier is a local-first tracker whose durable project state must be legible,
mergeable, and recoverable through Git. The previous milestone established
Markdown records in `.atelier-state/` and removed aggregate `manifest.json` and
`graph.json` files as canonical state.

The inherited Chainlink implementation still centers many commands on SQLite
rows and then uses export to render Markdown. That path is useful for migration,
but it keeps two stores competing to own canonical facts. It also means local
runtime database drift or schema breakage can block operations that should be
possible against known Markdown records.

Atelier still needs SQLite. Global queries, ready-work discovery, graph
traversal, workflow validation, search, and Mission Control projections should
not parse every Markdown file on every command.

## Decision

Atelier will use a Markdown-first persistence architecture:

- `RecordStore` owns canonical record files under `.atelier-state/`: discovery,
  parsing, validation, deterministic rendering, ID allocation, and atomic
  writes.
- `ProjectionIndex` owns rebuildable SQLite indexes derived from `RecordStore`
  records for global queries, graph traversal, search, validation, and Mission
  Control inputs.
- `RuntimeState` owns local-only `.atelier/` state such as current
  work/session association, local agent identity, and caches.

Successful canonical mutations write Markdown records first, then refresh or
mark stale the projection index. Normal durability must not depend on a later
SQLite-to-Markdown export.

Query commands may use SQLite after freshness checks. When a query would read a
stale projection, it must refresh/reindex when safe or fail with an actionable
rebuild or repair command.

`atelier export` and `atelier export --check` remain compatibility, repair, and
determinism-check commands during migration. Their target role is not to be the
ordinary step that makes successful mutations durable.

## Consequences

- Markdown record files are the source of truth for canonical project records.
- SQLite remains the fast query and runtime engine, but canonical records must
  be rebuildable from `.atelier-state/`.
- Command implementations must avoid adding new SQLite-first canonical mutation
  paths.
- Projection freshness becomes part of query correctness, not just handoff
  hygiene.
- Local runtime corruption must be distinguishable from canonical record loss in
  health checks.
- Fully equivalent SQLite/Markdown live-state synchronization is not the target
  architecture.
- A daemon is deferred until an interactive workflow proves that a long-lived
  process is needed for freshness or UI latency.

## Supersedes

This ADR refines earlier export/rebuild milestone language that described
mutating commands as updating exports after SQLite mutation. That behavior is a
transitional compatibility path, not the destination architecture.
