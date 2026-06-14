# Source Layout

Atelier is a single Rust crate rooted at `src/`. Do not look for a `crates/`
workspace split or old Chainlink-style module paths when orienting in the code.
The authoritative source map is the crate module tree declared from
[`src/lib.rs`](../../src/lib.rs) and [`src/main.rs`](../../src/main.rs).

## Current Modules

| Concern | Current files | Notes |
| --- | --- | --- |
| Command dispatch and CLI entrypoint | `src/main.rs`, `src/lib.rs`, `src/commands/mod.rs` | `src/main.rs` owns Clap parsing and top-level command routing. `src/commands/mod.rs` is the module registry for concrete command handlers. |
| Command-surface drift checks | `src/command_surface.rs` | Compares help text and docs against the implemented command surface. |
| Projection and rebuild | `src/projection_index.rs`, `src/commands/projection.rs`, `src/commands/rebuild.rs` | `projection_index.rs` owns rebuildable SQLite source metadata and freshness checks. The command modules wrap that behavior for CLI use. |
| Canonical record storage | `src/record_store.rs`, `src/record_store/record_kinds.rs`, `src/record_store/relationships.rs` | `RecordStore` owns canonical Markdown discovery, parsing, rendering, and deterministic mutation helpers. |
| Workflow policy | `src/workflow_policy.rs`, `src/commands/issue_workflow.rs`, `src/commands/workflow.rs` | Policy parsing, validation, and workflow checks live here rather than in ad hoc runtime files. |
| Evidence | `src/commands/evidence.rs`, `src/models.rs` | Evidence commands create, attach, show, and capture proof records using the shared evidence data types. |
| Worktree behavior | `src/commands/work.rs`, `src/commands/status.rs`, `src/commands/activity_log.rs` | Worktree start/abandon/status behavior is handled here, including work association updates and status rendering. |
| Durable layout helpers | `src/storage_layout.rs`, `src/command_storage.rs` | These modules locate `.atelier/`, runtime state, and canonical state paths. |
| Shared record and domain types | `src/models.rs`, `src/record_id.rs`, `src/utils.rs` | Common record structures, IDs, and formatting helpers used across the crate. |
| SQLite persistence | `src/db/` | Schema, record reads/writes, and projection-backed query operations. |

## What Not To Assume

- There is no multi-crate `crates/` tree.
- Command handlers live under `src/commands/*.rs`, not in per-command crates.
- `RecordStore`, `ProjectionIndex`, and workflow policy are first-class modules
  in the main crate, not hidden behind generated code or legacy package names.
- Worktree handling is split between command handlers and shared state helpers,
  not a separate daemon or shell wrapper.

## Orientation Order

When a scout needs the current implementation surface, read in this order:

1. [`src/main.rs`](../../src/main.rs) for CLI entrypoints and dispatch.
2. [`src/commands/mod.rs`](../../src/commands/mod.rs) for available command modules.
3. [`src/record_store.rs`](../../src/record_store.rs) for canonical Markdown ownership.
4. [`src/projection_index.rs`](../../src/projection_index.rs) for rebuildable SQLite projection state.
5. [`src/workflow_policy.rs`](../../src/workflow_policy.rs) for workflow-policy validation.
6. [`src/commands/evidence.rs`](../../src/commands/evidence.rs) and
   [`src/commands/work.rs`](../../src/commands/work.rs) for evidence and worktree behavior.
