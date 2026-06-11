# Architecture

This map separates current inherited implementation from target Atelier
architecture. Product intent lives in [SPEC.md](../../SPEC.md), domain language
lives in [CONTEXT.md](../../CONTEXT.md), and fork provenance is documented in
[Chainlink Provenance](provenance.md).

## Current Implementation

Atelier currently starts from the Chainlink Rust CLI:

- `src/main.rs`: Clap command routing and global CLI options.
- `src/commands/`: command handlers for the documented core CLI surface:
  issues, missions, plans, evidence, links, workflow validation, work/worktrees,
  canonical export/rebuild, import-beads, lint, and doctor.
- `src/db/`: SQLite schema, migrations, and persistence operations.
- `src/models.rs`: shared data structures.
- `resources/atelier/`: bundled rules, hook configuration, and integration
  assets renamed for Atelier from the inherited Chainlink defaults.
- `tests/`: CLI integration and smoke coverage.
- `fuzz/`: libFuzzer targets for CLI output, create, dependencies, import,
  search, and state-machine behavior.

See [Chainlink Provenance](provenance.md) for inherited module boundaries,
preservation expectations, and deferred migration areas.
See [Work Model](work-model.md) for mission, milestone, epic, issue, workflow
validator, and evidence relationships.
See [Milestone Records](milestone-records.md) for the first-class checkpoint
record contract and validation model.
See [CLI Surface Tiers](cli-surface.md) for the public-help, compatibility, and
integration command policy.
See [Human CLI Output](human-cli-output.md) for the detail, queue, hierarchy,
color, width, formatter, and test conventions for non-JSON command output.
See [Workflow Configuration Contract](workflow-configuration.md) for the
repository-owned workflow policy path, schema, validators, hooks, reload
behavior, and examples.
See [Markdown-First Record Store](markdown-first-record-store.md) for the
RecordStore, ProjectionIndex, and RuntimeState boundaries that govern durable
Markdown writes, rebuildable SQLite indexes, and local-only runtime data.
See [Mission Control TUI](mission-control-tui.md) for the read-only terminal UI
consumer contract, projection degradation rules, navigation model, and fixture
expectations.

Accepted ADRs record cross-cutting product decisions:

- [ADR 0001: Project-Scoped Random Record IDs](../adr/0001-project-scoped-random-record-ids.md)
- [ADR 0002: Markdown-First Record Store](../adr/0002-markdown-first-record-store.md)
- [ADR 0003: Evidence Artifact Storage](../adr/0003-evidence-artifact-storage.md)
- [ADR 0004: Work Association Replaces Default Lock Sync](../adr/0004-work-lock-sync-policy.md)

## Target Architecture

[SPEC.md](../../SPEC.md) defines the target architecture, using the vocabulary in
[CONTEXT.md](../../CONTEXT.md):

- `.atelier-state/` contains deterministic, mergeable Markdown records.
- SQLite is the fast local ProjectionIndex and RuntimeState store for queries,
  locks, sessions, workflow checks, and Mission Control inputs.
- Mutating commands are migrating toward Markdown-first writes through
  `RecordStore`, with SQLite refreshed as a rebuildable `ProjectionIndex`.
- `export --check` detects stale canonical records and derived projections.
- `rebuild` recreates SQLite projection state from committed Markdown records.
- First-class concepts include missions, milestone checkpoint records, issues,
  plans, evidence, runs, typed links, workflows, and workflow validators.
- Repository-owned workflow policy lives in `atelier.workflow.yaml`, while
  `.atelier-state/` remains the deterministic exported tracker projection.

## Boundaries

- CLI parsing should stay thin and delegate behavior to command and domain
  modules.
- Database code owns schema migration, transaction boundaries, and persistence
  invariants for projection and runtime tables.
- RecordStore code must own deterministic canonical Markdown serialization and
  record-local validation.
- ProjectionIndex code must own rebuild, reindex, query freshness, and
  stale-projection detection.
- Workflow validator evaluation should produce machine-readable results suitable
  for Mission Control.
- Mission Control TUI code should consume only the documented projection fields
  and keep CLI commands plus durable projections as the primary agent interface.
- Human CLI rendering should use shared detail, queue, and hierarchy formatter
  conventions while keeping canonical projection logic separate from display
  text.
- Git/worktree helpers should remain convenience layers over Git, not a
  replacement sync system.

## Architecture Risks

- Some inherited Chainlink concepts and module boundaries can still obscure
  Atelier target-state work.
- Backup-style export/import can be mistaken for canonical projection/rebuild.
- SQLite state must not become the only durable source once `.atelier-state/`
  exists.
- Process features must stay risk-scaled and configurable to avoid red tape.
