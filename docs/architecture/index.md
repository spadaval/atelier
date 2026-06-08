# Architecture

This map separates current inherited implementation from target Atelier
architecture. Product intent lives in [SPEC.md](../../SPEC.md), domain language
lives in [CONTEXT.md](../../CONTEXT.md), and fork provenance is documented in
[Chainlink Provenance](provenance.md).

## Current Implementation

Atelier currently starts from the Chainlink Rust CLI:

- `src/main.rs`: Clap command routing and global CLI options.
- `src/commands/`: command handlers for issues, sessions, milestones, locks,
  sync, export, import, and related operational flows.
- `src/db/`: SQLite schema, migrations, and persistence operations.
- `src/models.rs`: shared data structures.
- `resources/atelier/`: bundled rules, hook configuration, and integration
  assets renamed for Atelier from the inherited Chainlink defaults.
- `tests/`: CLI integration and smoke coverage.
- `fuzz/`: libFuzzer targets for CLI output, create, dependencies, import,
  search, and state-machine behavior.

See [Chainlink Provenance](provenance.md) for inherited module boundaries,
preservation expectations, and deferred migration areas.

## Target Architecture

[SPEC.md](../../SPEC.md) defines the target architecture, using the vocabulary in
[CONTEXT.md](../../CONTEXT.md):

- SQLite is the fast local runtime store for queries, locks, sessions,
  workflow checks, and Mission Control projections.
- `.atelier-state/` is the deterministic, mergeable repo projection.
- Mutating commands update canonical exports by default.
- `export --check` detects stale projections.
- `rebuild` recreates SQLite state from committed exported state.
- First-class concepts include missions, milestones, issues, plans, evidence,
  gates, runs, typed links, and workflows.

## Boundaries

- CLI parsing should stay thin and delegate behavior to command and domain
  modules.
- Database code owns schema migration, transaction boundaries, and persistence
  invariants.
- Export/rebuild code must own deterministic serialization and stale-projection
  detection.
- Workflow and gate evaluation should produce machine-readable results suitable
  for Mission Control.
- Git/worktree helpers should remain convenience layers over Git, not a
  replacement sync system.

## Architecture Risks

- Some inherited Chainlink concepts and module boundaries can still obscure
  Atelier target-state work.
- Backup-style export/import can be mistaken for canonical projection/rebuild.
- SQLite state must not become the only durable source once `.atelier-state/`
  exists.
- Process features must stay risk-scaled and configurable to avoid red tape.
