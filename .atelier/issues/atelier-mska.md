---
created_at: "2026-06-23T16:13:40.066192089+00:00"
id: "atelier-mska"
issue_type: "mission"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ckca"
    type: "advances"
  - kind: "issue"
    id: "atelier-hl1n"
    type: "advances"
  - kind: "issue"
    id: "atelier-v9sy"
    type: "advances"
  - kind: "issue"
    id: "atelier-x7lq"
    type: "advances"
  - kind: "issue"
    id: "atelier-xa9s"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "ready"
title: "Mission: Unify record storage and lazy domain cache"
updated_at: "2026-06-23T16:13:40.066192089+00:00"
---

## Description

Rework Atelier's persistence architecture around three explicit layers:

- record-file storage: shared Markdown/YAML file mechanics for all durable project records under `.atelier/`
- domain model: concrete Issue, Evidence, Review, and future domain types with their own invariants and behavior
- cache: lazy SQLite query index shaped like the domain model, rebuilt only when a command actually needs cache data

This is an experimental product migration. Sweeping schema and model changes are in scope. Do not preserve compatibility aliases, old projection terminology, generic-record cache tables, or eager refresh behavior unless a human explicitly adds that requirement.

The target design treats record files as project state and SQLite as a cache. Write paths should update record files without eagerly rebuilding cache state. Query paths should ask a cache boundary for data when needed; that boundary may rebuild or repair the cache at that point.

SQLite remains the target cache engine for this mission. A custom binary cache, Tantivy search index, graph database, or external search service may be reconsidered later, but this rewrite should first fix cache ownership, schema shape, lazy access, and incremental update behavior. The cache should be disposable: schema or cache-version mismatch should trigger rebuild, not long-lived migration work for ignored runtime state.

Incremental cache updates should be designed around changed record files. For Atelier-authored writes, the write path should know the exact record files it changed. For external edits, the cache boundary may use file metadata, Git status, and source metadata to find changed candidates, then reparse/reindex changed records directly. Hash checks are optional optimizations for noisy metadata, not the default prerequisite before parsing a changed candidate.

Constraints:
- Use cache terminology consistently in code, docs, and user-facing behavior; retire canonical/projection language where it describes ordinary record-file or cache behavior.
- Unify durable storage mechanics for issue, evidence, review, and future record files while keeping concrete domain types and domain services separate from storage abstractions.
- Rewrite the SQLite cache schema to resemble the domain query model, not the generic record-file storage model; prefer domain tables such as issues, evidence, reviews, links, labels, and cache source metadata.
- Remove eager post-write cache rebuild requirements; record writes may leave cache stale, and commands refresh cache only when they need cache data.
- Keep SQLite for this mission, but treat it as disposable cache. Do not design cache migrations as durable project-state migrations.
- Build incremental cache refresh around changed record-file reindexing with full rebuild as fallback; do not require a separate delta system before removing eager rebuilds.
- Current issue field behavior may remain as-is; future custom top-level fields are a later feature and should not force a generic JSON domain model now.

Risks:
- Broad storage, cache, and schema changes can break most command surfaces at once; sequence work through explicit migration slices with observable CLI proof.
- Lazy cache semantics can hide stale-cache bugs unless query commands consistently route through the cache boundary.
- Incremental update logic can drift from full rebuild behavior unless each domain record type has one cache-indexing path reused by both incremental and full rebuild flows.
- Terminology cleanup can become cosmetic churn; tie renames to behavior and boundary changes.

Validation:
- Record-file writes no longer call an eager refresh-after-write path, and a batch of relationship writes does not rebuild the full cache after each mutation.
- A command that needs cache data obtains it through the cache boundary and rebuilds or repairs stale cache on demand.
- Incremental cache repair reparses/reindexes a small changed-record candidate set and falls back to full rebuild for broad or uncertain changes.
- SQLite schema no longer uses the current hybrid issue-plus-generic-record table model as the target architecture.
- Issue, evidence, and review detail commands continue to load full domain content from record files while list/graph commands use cache rows.
- Docs and command output use cache terminology for SQLite-derived state.

## Outcome

Atelier has a unified record-file storage layer, concrete domain model boundaries for issue/evidence/review behavior, and a lazy SQLite cache whose schema and access patterns are shaped around domain queries rather than storage mechanics.
