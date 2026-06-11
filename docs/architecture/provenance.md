# Chainlink Provenance

Atelier begins as a fork of `dollspace-gay/chainlink`. This provenance is part
of the inherited architecture baseline, not the target product definition.
Target behavior is defined by [SPEC.md](../../SPEC.md), and target terminology
is defined by [CONTEXT.md](../../CONTEXT.md).

## Inherited Baseline

The Chainlink codebase supplies the working Rust CLI and local runtime machinery
that Atelier is evolving:

- Original `src/main.rs` and `src/commands/`: Clap-based command routing and
  command handlers for issue CRUD, list/search/show/update flows,
  dependencies, relations, comments, labels, milestones, archive, sessions,
  locks, sync, import/export, token usage, status, timers, and agent identity
  workflows.
- `src/db/`: SQLite schema management, migrations, and persistence operations
  for issues, comments, labels, dependencies, relations, milestones, sessions,
  archive records, time entries, and token usage.
- `src/models.rs`, `src/identity.rs`, `src/locks.rs`, `src/lock_check.rs`,
  `src/sync.rs`, and `src/utils.rs`: shared data types and operational helpers
  around identity, locks, sync, and CLI support behavior. Token accounting
  helpers were removed with the legacy usage command surface.
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

- Markdown records under `.atelier/` become the deterministic, mergeable
  repository record store that can rebuild local SQLite projections.
- SQLite remains the fast local ProjectionIndex and RuntimeState store for
  queries, locks, sessions, workflow checks, and Mission Control inputs.
- Mutating commands are migrating toward Markdown-first writes, and
  `export --check` detects stale canonical records and derived projections.
- `rebuild` recreates local SQLite projection state from committed Markdown
  records after checkout, pull, merge, or clone.
- Missions, milestone checkpoint records, issues, plans, evidence, workflow
  validators, runs, typed links, and workflows become first-class Atelier
  concepts instead of only inherited issue-tracker fields.
- Agent-facing commands keep focused human output for coordination and
  validation while durable projections provide machine-readable state.

Use "Chainlink" when documenting source provenance or current behavior that is
still plainly inherited from the original project. Use "Atelier" when
documenting target product behavior or new architecture decisions.

## Preservation Expectations

Inherited behavior should be preserved until assigned tracker work or an ADR
explicitly changes it:

- Preserve useful CLI behavior, SQLite persistence invariants, sessions used by
  current work association, hooks, durable projection behavior, and practical
  test coverage while rename and migration work is underway.
- Do not replace working inherited modules with compatibility shims whose only
  purpose is hiding current names before target behavior exists.
- Do not treat backup-oriented Chainlink export/import as the target canonical
  projection and rebuild system.
- When replacing inherited behavior, update target-state docs or ADRs so the
  new design does not rely only on historical prose.
- If inherited tests or resources no longer apply, retire them only through the
  tracker work that owns that migration and record the reason in the relevant
  docs or tracker handoff.

## Deferred Migration Areas

The following areas are intentionally deferred unless a specific bead assigns
them:

- Completing any remaining package, binary, resource path, and user-facing
  rename work.
- Moving local runtime state fully from inherited Chainlink conventions toward
  `.atelier/runtime/` and `.atelier/cache/`.
- Replacing backup-style export/import with canonical `.atelier/`
  Markdown records and rebuildable projections.
- Adding first-class missions, plans, evidence, workflow validators, runs, typed
  links, and workflow configuration.
- Reworking lock sync behavior beyond what the relevant migration or design
  issue decides.

Legacy command preservation has been superseded by the command-surface
simplification mission: non-core aliases and inherited utility groups should be
deleted rather than hidden behind compatibility shims.
