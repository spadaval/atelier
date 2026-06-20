# Architecture

This map covers implementation architecture for Atelier's target layered Cargo
workspace: crate ownership, persistence boundaries, local projection state, and
inherited Chainlink structure being migrated out of the root package. Product behavior lives in
[Product](../product/index.md), product intent lives in [SPEC.md](../../SPEC.md),
domain language lives in [CONTEXT.md](../../CONTEXT.md), and fork provenance is
documented in [Chainlink Provenance](provenance.md).

## Target Workspace

Atelier is migrating to a virtual-root Cargo workspace:

- [Source Layout](source-layout.md): the target crate map for
  `atelier-core`, `atelier-workflow`, `atelier-records`, `atelier-sqlite`,
  `atelier-app`, and `atelier-cli`, plus the migration map from the current
  root package.
- [SQLite Projection Schema](sqlite-runtime-schema.md): the target rebuildable
  projection tables and the exclusion of non-Markdown runtime tables.
- `crates/atelier-cli`: owns the `atelier` binary, Clap parser, terminal
  rendering, dispatch telemetry, and exit-code mapping.
- `crates/atelier-app`: owns use-case orchestration through request, outcome,
  and view-model APIs that the CLI renders.
- `crates/atelier-sqlite`: owns rebuildable projection SQLite schema/query
  code.
- `crates/atelier-records`, `crates/atelier-workflow`, and
  `crates/atelier-core`: own canonical Markdown storage, workflow policy, and
  pure domain types.
- `crates/atelier-cli/tests` and `fuzz/`: migrate toward the crate that owns
  the invariant under test while preserving CLI integration coverage for
  terminal behavior.

See [Chainlink Provenance](provenance.md) for inherited module boundaries,
preservation expectations, and deferred migration areas.
See [Markdown-First Record Store](markdown-first-record-store.md) for the
RecordStore and ProjectionIndex boundaries that govern durable Markdown writes,
rebuildable SQLite indexes, and local-only diagnostics/cache data.
See [Local Command Diagnostics](local-command-diagnostics.md) for the global
local diagnostics store, command telemetry fields, redaction defaults, opt-out
controls, retention behavior, and Mission Control export boundary.
See [Product](../product/index.md) for the work model, milestone contract,
public CLI surface, human output, workflow policy, and Mission Control
experience that the architecture supports.

Accepted ADRs record cross-cutting product choices:

- [ADR 0001: Project-Scoped Random Record IDs](../adr/0001-project-scoped-random-record-ids.md)
- [ADR 0002: Markdown-First Record Store](../adr/0002-markdown-first-record-store.md)
- [ADR 0003: Evidence Artifact Storage](../adr/0003-evidence-artifact-storage.md)
- [ADR 0004: Work Association Replaces Default Lock Sync](../adr/0004-work-lock-sync-policy.md)
- [ADR 0009: Virtual Workspace Root And CLI-Owned Binary](../adr/0009-virtual-workspace-root-and-cli-binary.md)
- [ADR 0010: Session-As-Issue-Events And PR Coordination Boundaries](../adr/0010-session-aware-pr-coordination-boundaries.md)
- [ADR 0011: Native Review Modes And Room Authority](../adr/0011-native-review-modes-and-room-authority.md)
- [ADR 0013: Workflow Transition Actions And Branching](../adr/0013-workflow-transition-actions-and-branching.md)
- [ADR 0014: Status Role Attribution Replaces Sessions](../adr/0014-status-role-attribution-replaces-sessions.md)

## Target Architecture

[SPEC.md](../../SPEC.md) defines the target architecture, using the vocabulary in
[CONTEXT.md](../../CONTEXT.md):

- `.atelier/` is the single project state root. It contains deterministic,
  mergeable Markdown records and tracked project config.
- `.atelier/runtime/` and `.atelier/cache/` are ignored local locations for the
  SQLite ProjectionIndex, locks, diagnostics, workflow checks, Mission Control
  inputs, and UI caches. SQLite tracker tables must be rebuildable from
  canonical Markdown.
- Command diagnostics are local-only telemetry outside the canonical record
  directories and do not create exported run/session records until a later
  projection contract explicitly opts in.
- Mutating commands are migrating toward Markdown-first writes through
  `RecordStore`, with SQLite refreshed as a rebuildable `ProjectionIndex`.
- The root package is being deleted in favor of a virtual workspace root; all
  executable ownership moves to `crates/atelier-cli`.
- `doctor` and `lint` detect stale, invalid, or missing tracker state through
  operator-facing health checks.
- `doctor --fix` repairs ignored local projection/cache state from committed
  Markdown records when it is safe to do so.
- First-class concepts include missions, issues, evidence, typed links,
  workflows, workflow validators, status roles, review artifacts, and deferred
  run metadata; their user-visible behavior is defined in [Product](../product/index.md).
- Repository-owned issue workflow policy lives at the fixed
  `.atelier/workflow.yaml` path. Loading, schema validation, status categories,
  transition lookup, validator evaluation, guidance rendering, and branch
  lifecycle policy belong to `atelier-workflow`; app and CLI layers consume
  those APIs instead of maintaining parallel policy copies.

## Boundaries

- CLI parsing should stay thin and delegate behavior to command and domain
  modules.
- Database code owns schema migration, transaction boundaries, and persistence
  invariants for projection tables.
- RecordStore code must own deterministic canonical Markdown serialization and
  record-local validation.
- ProjectionIndex code must own rebuild, reindex, query freshness, and
  stale-projection detection.
- Workflow validator evaluation should be implemented in `atelier-workflow` and
  produce machine-readable results suitable for app orchestration, product
  workflow surfaces, and Mission Control.
- Mission Control TUI code should consume only documented projection fields and
  keep CLI commands plus durable projections as the primary agent interface.
- Human CLI rendering should keep canonical projection logic separate from
  display text.
- Git/worktree helpers should remain convenience layers over Git, not a
  replacement sync system.

## Architecture Risks

- Some inherited Chainlink concepts and module boundaries can still obscure
  Atelier target-state work.
- Backup-style export/import can be mistaken for canonical projection/rebuild.
- SQLite state must not become the only durable source once tracked `.atelier/`
  canonical records exist.
- Process features must stay risk-scaled and configurable to avoid red tape.
