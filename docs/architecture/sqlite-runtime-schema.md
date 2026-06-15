# SQLite Runtime Schema

Atelier uses one local database at `.atelier/runtime/state.db`. Canonical
Markdown remains the source of truth; deleting or rebuilding this database must
not lose tracked work.

## Ownership

Projection tables are rebuildable from `.atelier/` Markdown:

- `issues`
- `labels`
- `dependencies`
- `relations`
- `records`
- `record_links`
- `projection_index_sources`

Runtime tables hold ignored local diagnostics or cache metadata:

- `runtime_metadata`

The target schema does not include `sessions`, `work_associations`, hidden
claims, or active-work source-of-truth tables. Current work is derived from
canonical issue status and checkout context.

Old SQLite schema compatibility is out of scope for the crate rewrite. The
supported migration path is `atelier doctor --fix` or rebuild from committed
Markdown records.
