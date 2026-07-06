# SQLite Projection Schema

Atelier uses one ignored local database at `.atelier/runtime/state.db`.
Canonical Markdown remains the source of truth; deleting or rebuilding this
database must not lose tracked work.

## Ownership

All target SQLite tables are projection tables rebuildable from `.atelier/`
Markdown:

- `issues`
- `labels`
- `dependencies`
- `relations`
- `records`
- `record_labels`
- `record_links`
- `evidence`
- `plans`
- `milestones`
- `projection_sources`

The target schema does not include `runtime_metadata`, `sessions`,
`work_associations`, hidden claims, active-work source-of-truth tables, or other
non-Markdown tracker facts. Current work is derived from canonical issue status
and checkout context.

Old SQLite schema compatibility is out of scope for the crate rewrite. The
supported migration path is `atelier check --fix` or rebuild from committed
Markdown records.
