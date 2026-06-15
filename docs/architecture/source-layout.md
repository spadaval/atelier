# Source Layout

Atelier is currently implemented as a single Rust crate rooted at `src/`, but
the target architecture is a layered Cargo workspace. Treat the current
single-crate layout as transitional implementation state, not the destination
for crate rewrite work.

The durable workspace decision is
[ADR 0008: Layered Cargo Workspace](../adr/0008-layered-cargo-workspace.md).
The active/current-work source-of-truth update is recorded in
[ADR 0004: Work Association Replaces Default Lock Sync](../adr/0004-work-lock-sync-policy.md).

## Current Single-Crate Layout

Until extraction lands, the authoritative implementation map is the crate module
tree declared from [`src/lib.rs`](../../src/lib.rs) and
[`src/main.rs`](../../src/main.rs).

| Concern | Current files | Notes |
| --- | --- | --- |
| Command dispatch and CLI entrypoint | `src/main.rs`, `src/lib.rs`, `src/commands/mod.rs` | `src/main.rs` owns Clap parsing and top-level command routing. `src/commands/mod.rs` is the module registry for concrete command handlers. |
| Command-surface drift checks | `src/command_surface.rs` | Compares help text and docs against the implemented command surface. |
| Projection and rebuild | `src/projection_index.rs`, `src/commands/projection.rs`, `src/commands/rebuild.rs` | `projection_index.rs` owns rebuildable SQLite source metadata and freshness checks. The command modules wrap that behavior for CLI use. |
| Canonical record storage | `src/record_store.rs`, `src/record_store/record_kinds.rs`, `src/record_store/relationships.rs` | `RecordStore` owns canonical Markdown discovery, parsing, rendering, and deterministic mutation helpers. |
| Workflow policy | `src/workflow_policy.rs`, `src/commands/issue_workflow.rs`, `src/commands/workflow.rs` | Policy parsing, validation, and workflow checks live here rather than in ad hoc runtime files. |
| Evidence | `src/commands/evidence.rs`, `src/models.rs` | Evidence commands create, attach, show, and capture proof records using the shared evidence data types. |
| Worktree behavior | `src/commands/work.rs`, `src/commands/status.rs`, `src/commands/activity_log.rs` | Worktree start/abandon/status behavior is handled here while runtime active-work state is being removed as a source of truth. |
| Durable layout helpers | `src/storage_layout.rs`, `src/command_storage.rs` | These modules locate `.atelier/`, runtime state, and canonical state paths. |
| Shared record and domain types | `src/models.rs`, `src/record_id.rs`, `src/utils.rs` | Common record structures, IDs, and formatting helpers used across the crate. |
| SQLite persistence | `src/db/` | Current schema, record reads/writes, and projection-backed query operations before the SQLite rewrite replaces inherited persistence boundaries. |

## Target Workspace

The crate rewrite introduces these internal workspace crates:

| Crate | Owns | Must not own |
| --- | --- | --- |
| `atelier-core` | Record IDs, domain record data types, relationship values, status/value validation primitives, and shared constants. | Filesystem traversal, SQLite, Clap, telemetry, command rendering, or runtime process state. |
| `atelier-workflow` | Workflow policy parsing, status/category interpretation, transition validation, and workflow validator primitives on top of `atelier-core`. | Canonical Markdown writes, SQLite tables, CLI rendering, or command orchestration. |
| `atelier-records` | Canonical Markdown record discovery, parsing, validation, deterministic rendering, ID allocation, atomic writes, relationship rendering, and activity sidecars. | Global query planning, runtime active-work truth, local cache ownership, or command view models. |
| `atelier-sqlite` | Rebuildable `ProjectionIndex` tables, projection freshness, graph/search/ready queries, and local-only `RuntimeState` tables. | Canonical project facts that cannot be rebuilt from committed `.atelier/` records. |
| `atelier-app` | Use-case orchestration, command handlers, service ports, view models, and coordination between records, workflow, SQLite, runtime state, Git, and diagnostics. | Clap definitions, process entrypoint behavior, or low-level record parsing duplicated from `atelier-records`. |
| `atelier-cli` | Clap definitions, terminal setup, process exit mapping, and thin delegation into `atelier-app`. | Product workflow logic, storage mutation rules, or duplicate command implementations. |

Dependency direction is one-way:

```text
atelier-cli -> atelier-app -> {atelier-records, atelier-sqlite, atelier-workflow} -> atelier-core
```

Lower crates must not depend on `atelier-app` or `atelier-cli`, and no crate may
introduce a cycle to preserve an old module path. Rust crate APIs are internal
to this repository; old `atelier::...` paths and temporary crate APIs do not
carry compatibility promises.

## Temporary Internal Adapters

A temporary internal migration adapter is rewrite scaffolding that lets one
layer call another layer while extraction is incomplete. It is allowed only when
all of these are true:

- the adapter is internal to repository code and does not add a public CLI
  command, public Rust compatibility facade, old-command alias, fallback reader,
  or old-path re-export;
- the owning issue is named in the issue record or in a nearby code/doc marker;
- the removal condition is explicit enough for a later worker to know when to
  delete it; and
- the adapter can be found by closeout inventory, preferably with a marker such
  as `MIGRATION_ADAPTER(issue=<id>, remove_when=<condition>)` near the adapter.

Mission closeout must review remaining adapter markers before the rewrite can
close. Once an adapter's removal condition is met, cleanup should remove it
directly rather than add a staged deprecation path.

## Orientation Order

When a scout needs the current implementation surface during the rewrite, read
in this order:

1. [ADR 0008](../adr/0008-layered-cargo-workspace.md) for target crate
   boundaries and dependency direction.
2. [ADR 0004](../adr/0004-work-lock-sync-policy.md) for status-derived current
   work and runtime-association limits.
3. [`src/main.rs`](../../src/main.rs) for current CLI entrypoints and dispatch.
4. [`src/commands/mod.rs`](../../src/commands/mod.rs) for current command
   modules.
5. [`src/record_store.rs`](../../src/record_store.rs) for canonical Markdown
   ownership before `atelier-records` extraction.
6. [`src/projection_index.rs`](../../src/projection_index.rs) for rebuildable
   SQLite projection state before `atelier-sqlite` extraction.
7. [`src/workflow_policy.rs`](../../src/workflow_policy.rs) for workflow-policy
   validation before `atelier-workflow` extraction.
