# ADR 0009: Virtual Workspace Root And CLI-Owned Binary

## Status

Accepted.

## Context

Atelier is finishing a migration from the inherited Chainlink single-crate CLI
into layered internal crates. The repository needs the layered workspace to be
the only target architecture, not a scaffold that continues to coexist with a
root `atelier` package.

Keeping a root package while adding crates would preserve old `atelier::...`
paths, make ownership ambiguous, and let command handlers, storage, workflow
policy, and terminal rendering continue to depend on each other through
accidental module access. The mission closeout instead requires deletion of the
root crate, a warning-free workspace check, and no old root re-export paths.

## Decision

The repository root becomes a virtual Cargo workspace root. It owns workspace
membership, shared package metadata, lints, and profiles only.

The `atelier` executable is owned by `crates/atelier-cli`. `atelier-cli` owns
Clap parser definitions, terminal rendering, dispatch telemetry, and process
exit mapping. It delegates product behavior to `atelier-app`.

The target crate layers are:

1. `atelier-core` for pure domain types and shared vocabulary.
2. `atelier-workflow` for repository workflow policy and transition checks.
3. `atelier-records` for canonical Markdown record storage.
4. `atelier-sqlite` for rebuildable projection and ignored runtime SQLite
   state.
5. `atelier-app` for use-case orchestration, request/outcome APIs, and
   view-models.
6. `atelier-cli` for the public binary and terminal shell.

Temporary adapters are internal migration aids only. They must have explicit
removal owners before the root-deletion epic can close, and they must not
preserve root package compatibility aliases or old `atelier::...` re-export
paths unless a human explicitly approves a compatibility window.

## Consequences

- Implementation epics can assign ownership by crate instead of by old root
  module paths.
- CLI behavior remains user-visible through the `atelier` binary, but the
  binary package moves to `crates/atelier-cli`.
- The root `Cargo.toml` no longer provides library or binary targets after
  closeout.
- Tests and fuzz targets move toward the crate that owns the invariant under
  test, while CLI integration coverage remains with the CLI shell.
- Warning-free closeout must run against the workspace, not a root package.

## Alternatives Considered

### Keep A Root Package That Re-exports New Crates

This would reduce short-term churn for internal imports. It was rejected
because the crate APIs are internal, compatibility is not a requirement, and a
root re-export package would hide incomplete migration work.

### Keep The Binary At The Root

This would preserve the old package entrypoint while still allowing library
crates. It was rejected because command parsing and terminal rendering need a
clear owner, and `atelier-cli` is the natural crate boundary for the public
binary.

### Use One `atelier-app` Crate For Everything Below The CLI

This would reduce workspace complexity. It was rejected because canonical
Markdown storage, workflow policy, SQLite projection/runtime state, and pure
domain vocabulary need separately testable boundaries during the migration.
