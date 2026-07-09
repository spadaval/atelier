# Chainlink Provenance

Atelier begins as a fork of `dollspace-gay/chainlink`. This provenance is part
of the inherited architecture baseline, not the target product definition.
Target direction is described by
[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md), and target terminology is defined
by [CONTEXT.md](../../CONTEXT.md).

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
- `resources/atelier/`: renamed rule assets that descend from Chainlink
  resource content. Claude Code hook, MCP, and integration assets were removed
  from the target Atelier product surface.
- `tests/` and `fuzz/`: inherited CLI integration, smoke, property, and fuzz
  coverage for current behavior.

These pieces are the implementation baseline for Milestone 1. Their presence
does not make Chainlink names or backup-style export/import semantics the
Atelier target state.

## Target Atelier Direction

Atelier is not a thin rename. The product direction in
[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md) keeps useful Chainlink runtime
machinery while changing the durable product model:

- Markdown records under `.atelier/` are the deterministic, mergeable record
  store from which local query state can be rebuilt.
- SQLite at `.atelier/runtime/state.db` is a disposable domain cache containing
  selected query facts. It does not own locks, sessions, durable workflow
  state, or complete record bodies.
- Mutating commands write concrete record files and leave changed cache facts
  for the next cache-backed query to repair lazily through `CacheManager`.
- Hidden/admin cache diagnostics may verify deterministic rendering or local
  repair, and `check --fix` owns explicit ignored-state repair from committed
  record files after checkout, pull, merge, or clone.
- Missions, issues, evidence, workflow validators, runs, typed links, and
  workflows become first-class Atelier concepts instead of only inherited
  issue-tracker fields. Plan and checkpoint intent remains prose or referenced
  Markdown until a future contract reintroduces first-class records.
- Agent-facing commands keep focused human output for coordination and
  validation while the disposable cache accelerates domain queries.

Use "Chainlink" when documenting source provenance or current behavior that is
still plainly inherited from the original project. Use "Atelier" when
documenting target product behavior or new architecture choices.

## Preservation Expectations

Inherited behavior should be preserved until assigned tracker work or an ADR
explicitly changes it:

- Preserve useful CLI behavior, record-file persistence invariants, local
  runtime behavior that remains in scope, and practical test coverage while
  rename and migration work is underway.
- Do not replace working inherited modules with compatibility shims whose only
  purpose is hiding current names before target behavior exists.
- Do not treat backup-oriented Chainlink export/import as a target persistence
  or cache-rebuild system.
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
- Any remaining cleanup of backup-style export/import migration diagnostics.
- Adding first-class missions, evidence, workflow validators, runs, typed links,
  and workflow configuration.
- Reworking lock sync behavior beyond what the relevant migration or design
  issue decides.

Legacy command preservation has been superseded by the command-surface
simplification mission: non-core aliases and inherited utility groups should be
deleted rather than hidden behind compatibility shims.
