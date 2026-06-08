# Chainlink Provenance

Atelier begins as a fork of `dollspace-gay/chainlink`. This provenance is part
of the inherited architecture baseline, not the target product definition.
Target behavior is defined by [SPEC.md](../../SPEC.md), and target terminology
is defined by [CONTEXT.md](../../CONTEXT.md).

## Inherited Baseline

The Chainlink codebase supplies the working Rust CLI and local runtime machinery
that Atelier is evolving:

- `src/main.rs` and `src/commands/`: Clap-based command routing and command
  handlers for issue CRUD, list/search/show/update flows, dependencies,
  relations, comments, labels, milestones, archive, sessions, locks, sync,
  import/export, token usage, status, timers, and agent identity workflows.
- `src/db/`: SQLite schema management, migrations, and persistence operations
  for issues, comments, labels, dependencies, relations, milestones, sessions,
  archive records, time entries, and token usage.
- `src/models.rs`, `src/identity.rs`, `src/locks.rs`, `src/lock_check.rs`,
  `src/sync.rs`, `src/token_usage.rs`, and `src/utils.rs`: shared data types and
  operational helpers around identity, locks, sync, token accounting, and CLI
  support behavior.
- `resources/atelier/` and `resources/claude/`: renamed resource, rule, hook,
  MCP, and integration assets that descend from Chainlink resource content.
- `tests/` and `fuzz/`: inherited CLI integration, smoke, property, and fuzz
  coverage for current behavior.

These pieces are the implementation baseline for Milestone 1. Their presence
does not make Chainlink names or backup-style export/import semantics the
Atelier target state.

## Target Atelier Direction

Atelier is not a thin rename. The target architecture in [SPEC.md](../../SPEC.md)
keeps useful Chainlink runtime machinery while changing the durable product
model:

- SQLite remains the fast local runtime store for queries, locks, sessions,
  workflow checks, and Mission Control projections.
- `.atelier-state/` becomes the deterministic, mergeable repository projection
  that can rebuild local SQLite state.
- Mutating commands update canonical exports by default, and `export --check`
  detects stale projections.
- `rebuild` recreates local SQLite state from committed exported state after
  checkout, pull, merge, or clone.
- Missions, milestones, issues, plans, evidence, gates, runs, typed links, and
  workflows become first-class Atelier concepts instead of only inherited
  issue-tracker fields.
- Agent-facing commands keep stable JSON output where they support
  coordination, validation, automation, or Mission Control.

Use "Chainlink" when documenting source provenance or current behavior that is
still plainly inherited from the original project. Use "Atelier" when
documenting target product behavior or new architecture decisions.

## Preservation Expectations

Inherited behavior should be preserved until an assigned bead or ADR explicitly
changes it:

- Preserve useful CLI behavior, SQLite persistence invariants, sessions, locks,
  hooks, token usage accounting, JSON output, and practical test coverage while
  rename and migration work is underway.
- Do not replace working inherited modules with compatibility shims whose only
  purpose is hiding current names before target behavior exists.
- Do not treat backup-oriented Chainlink export/import as the target canonical
  projection and rebuild system.
- When replacing inherited behavior, update target-state docs or ADRs so the
  new design does not rely only on historical prose.
- If inherited tests or resources no longer apply, retire them only through the
  bead that owns that migration and record the reason in the relevant docs or
  tracker handoff.

## Deferred Migration Areas

The following areas are intentionally deferred unless a specific bead assigns
them:

- Completing any remaining package, binary, resource path, and user-facing
  rename work.
- Moving local runtime state fully from inherited Chainlink conventions toward
  `.atelier/`.
- Replacing backup-style export/import with canonical `.atelier-state/`
  projection and rebuild.
- Adding first-class missions, plans, evidence, gates, runs, typed links, and
  workflow configuration.
- Reworking lock sync behavior beyond what the relevant migration or design bead
  decides.
