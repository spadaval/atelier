# Source Layout

Atelier's target source layout is a Cargo workspace with a virtual repository
root and internal crates under `crates/`. The root `Cargo.toml` owns workspace
membership and shared package metadata only; it must not define a root library
or binary package after the crate migration closes. The `atelier` executable is
owned by `crates/atelier-cli`.

The current single-crate `src/` tree is migration input. During the rewrite,
agents should use this document as the destination map and treat remaining
root-package modules as code waiting to move behind the target crate boundary.

## Target Workspace

| Crate | Responsibility | May depend on |
| --- | --- | --- |
| `atelier-core` | Shared domain types, record IDs, typed relationships, workflow status/category vocabulary, and pure formatting helpers that do not touch the filesystem, SQLite, Clap, or Git. | External pure utility crates only. |
| `atelier-workflow` | Repository-owned workflow policy loading, transition validation, readiness checks, and guidance text evaluation. | `atelier-core`. |
| `atelier-records` | Canonical `.atelier/` Markdown record discovery, parsing, deterministic rendering, ID allocation, relationship rendering, and atomic tracked-file mutation. | `atelier-core`, `atelier-workflow` when validation needs workflow policy. |
| `atelier-sqlite` | Rebuildable SQLite `ProjectionIndex` and local `RuntimeState` schema/query code for projection freshness, graph/search/list/status queries, diagnostics, and ignored runtime/cache recovery. | `atelier-core`, `atelier-records`, `atelier-workflow`. |
| `atelier-app` | Use-case orchestration for mission, issue, evidence, workflow, status, doctor, export, lint, worktree, branch, and rebuild operations. It exposes request, outcome, and view-model APIs that are independent of Clap rendering. | `atelier-core`, `atelier-records`, `atelier-workflow`, `atelier-sqlite`. |
| `atelier-cli` | The `atelier` binary, Clap parser, command dispatch telemetry, terminal rendering, process exit mapping, and CLI-only transcript formatting. | `atelier-app`, plus lower crates only for types explicitly re-exported by `atelier-app` contracts. |

## Layering Rules

- Dependency direction flows from CLI to app to storage/workflow/domain crates.
  Lower crates must not call into `atelier-cli` or depend on Clap rendering.
- Rust crate APIs are internal to this repository. They may be replaced during
  the migration when doing so makes the crate boundary clearer.
- Tests and fuzz targets should move toward the lowest crate that owns the
  invariant being tested. CLI integration tests should stay in `atelier-cli`
  only when terminal behavior, Clap rejection, or exit-code mapping is the
  behavior under test.
- Temporary adapters are allowed only when tracked by explicit removal work.
  They must not become compatibility aliases, old-path re-exports, or staged
  deprecations unless a human explicitly requests a compatibility window.

## Temporary Adapter Policy

A temporary adapter is an internal shim that lets one migration slice compile
while its caller or callee is still moving to the target crate boundary.
Adapters are permitted only inside implementation-owned modules and only when
the owning issue records all of the following:

- the adapter name or search marker;
- the issue that owns removal;
- the condition that makes removal possible; and
- proof that the adapter is not a public CLI behavior, public Rust API promise,
  root-package re-export, old-command alias, or fallback compatibility path.

Closeout for the crate migration must inventory temporary adapters before
claiming the root package is deleted. Any remaining adapter must either be
removed, linked to an open removal issue that blocks closeout, or explicitly
classified as not an adapter because it is target-state code.

## Current Migration Map

| Current root area | Target owner |
| --- | --- |
| `src/models.rs`, `src/record_id.rs`, pure shared helpers | `atelier-core` |
| `src/workflow_policy.rs`, `src/commands/issue_workflow.rs`, `src/commands/workflow.rs` policy internals | `atelier-workflow` |
| `src/record_store.rs`, `src/record_store/` | `atelier-records` |
| `src/projection_index.rs`, `src/db/`, projection/rebuild storage internals | `atelier-sqlite` |
| `src/commands/*.rs` use-case logic, doctor/lint/export/status orchestration | `atelier-app` |
| `src/main.rs`, Clap definitions, terminal output, command-surface tracing | `atelier-cli` |

## Orientation Order

When a scout needs to orient during the migration, read in this order:

1. This target workspace map and [ADR 0009](../adr/0009-virtual-workspace-root-and-cli-binary.md).
2. [`CONTEXT.md`](../../CONTEXT.md) for domain vocabulary.
3. The current `src/` modules only as migration input for the target owner
   named above.
4. The child tracker issue for the crate being moved, because it names the
   current proof and any temporary adapter removal owner.
