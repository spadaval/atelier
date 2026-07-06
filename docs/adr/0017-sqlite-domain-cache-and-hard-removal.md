# ADR 0017: SQLite Domain Cache And Hard Removal

## Status

Accepted.

## Context

Atelier needs fast global query and validation paths, but its existing target
language allows a generic record/projection hybrid that risks becoming a second
project-state model. The repository is experimental, and the persistence
rewrite must not carry obsolete SQLite compatibility indefinitely.

## Decision

Record files are durable project state. Concrete domain types own parsing and
full record content. SQLite at `.atelier/runtime/state.db` is a disposable,
lazy domain cache whose tables contain only selected facts required by common
filtering, sorting, traversal, validation, and lookup paths.

Full rebuild and incremental repair share the same per-record-type indexers.
Commands check freshness lazily and repair a changed record-file source when
safe; they fall back to a full rebuild or actionable record-file error rather
than return known-stale cache data for decisions that affect orchestration,
validation, or closeout.

Cache schema/version mismatch is rebuildable ignored state. The rewrite may
hard-remove obsolete projection/canonical terminology where it describes
persistence, generic-record cache structures, SQLite-first or dual-write
paths, old cache migrations, and compatibility aliases. No compatibility window
is required for local cache files.

## Alternatives Considered

- **Custom binary cache:** rejected because SQLite already provides reliable
  local transactions, indexing, inspection, and rebuild behavior without a
  new format or query engine.
- **Tantivy:** rejected because text search alone does not cover relational
  ready-work, evidence, review, and workflow-validation lookups.
- **Graph database:** rejected because the bounded local relationship indexes
  do not justify an additional database service or operational model.
- **External search service:** rejected because it breaks local-first,
  offline, disposable-cache operation and adds deployment authority outside
  the repository.
- **Generic record mirror with compatibility migrations:** rejected because it
  duplicates domain ownership, requires broad schema understanding, and
  preserves obsolete persistence paths without user value.

## Consequences

- New cache tables need a named domain and a documented query need.
- Detail views load complete content from record files.
- Cache corruption or version drift is repaired by rebuild, not durable data
  migration.
- Implementers may delete obsolete cache paths directly once record-file
  rebuild coverage proves the replacement.
