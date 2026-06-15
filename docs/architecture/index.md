# Architecture

This map covers implementation architecture for Atelier's current single Rust
crate and target layered Cargo workspace: code ownership, persistence
boundaries, local runtime state, and inherited Chainlink structure. Product
behavior lives in [Product](../product/index.md), product intent lives in
[SPEC.md](../../SPEC.md), domain language lives in
[CONTEXT.md](../../CONTEXT.md), and fork provenance is documented in
[Chainlink Provenance](provenance.md).

## Current Implementation

Atelier is currently a single Rust crate while the crate rewrite moves toward
the layered workspace accepted in
[ADR 0008](../adr/0008-layered-cargo-workspace.md):

- [Source Layout](source-layout.md): the current module map and target
  workspace map for command dispatch, projection, `RecordStore`, workflow
  policy, evidence, worktree behavior, crate dependency direction, and temporary
  adapter policy.
- `src/db/`: SQLite schema, migrations, and persistence operations.
- `src/models.rs`: shared data structures.
- `resources/atelier/`: inherited bundled rule assets. Core `atelier init`
  does not copy these into repositories.
- `tests/`: CLI integration and smoke coverage.
- `fuzz/`: libFuzzer targets for CLI output, create, dependencies, import,
  search, and state-machine behavior.

See [Chainlink Provenance](provenance.md) for inherited module boundaries,
preservation expectations, and deferred migration areas.
See [Markdown-First Record Store](markdown-first-record-store.md) for the
RecordStore, ProjectionIndex, and RuntimeState boundaries that govern durable
Markdown writes, rebuildable SQLite indexes, and local-only runtime data.
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
- [ADR 0008: Layered Cargo Workspace](../adr/0008-layered-cargo-workspace.md)

## Target Architecture

[SPEC.md](../../SPEC.md) defines the target architecture, using the vocabulary in
[CONTEXT.md](../../CONTEXT.md):

- `.atelier/` is the single project state root. It contains deterministic,
  mergeable Markdown records and tracked project config.
- `.atelier/runtime/` and `.atelier/cache/` are ignored local state for the
  SQLite ProjectionIndex, locks, sessions, diagnostics, identity, workflow
  checks, Mission Control inputs, and UI caches.
- Command diagnostics are local-only telemetry outside the canonical record
  directories and do not create exported run/session records until a later
  projection contract explicitly opts in.
- Mutating commands are migrating toward Markdown-first writes through
  `RecordStore`, with SQLite refreshed as a rebuildable `ProjectionIndex`.
- The Rust implementation is migrating toward internal crates:
  `atelier-core`, `atelier-workflow`, `atelier-records`, `atelier-sqlite`,
  `atelier-app`, and `atelier-cli`. The visible binary remains `atelier`.
- Current work is derived from canonical issue workflow status, mission/epic
  graph links, and Git checkout context. Runtime active-work and claim state are
  local migration residue or ergonomic hints, not durable source of truth.
- `doctor` and `lint` detect stale, invalid, or missing tracker state through
  operator-facing health checks.
- `doctor --fix` repairs ignored local projection/runtime state from committed
  Markdown records when it is safe to do so.
- First-class concepts include missions, milestone checkpoint records, issues,
  plans, evidence, runs, typed links, workflows, and workflow validators; their
  user-visible behavior is defined in [Product](../product/index.md).
- Repository-owned workflow policy lives in `.atelier/config.toml` or a
  documented workflow policy file selected by config.

## Boundaries

- CLI parsing should stay thin and delegate behavior to command and domain
  modules.
- Internal crate dependencies should flow from `atelier-cli` to `atelier-app` to
  records/SQLite/workflow crates and then to `atelier-core`; lower crates must
  not depend on app or CLI layers.
- Database code owns schema migration, transaction boundaries, and persistence
  invariants for projection and runtime tables.
- RecordStore code must own deterministic canonical Markdown serialization and
  record-local validation.
- ProjectionIndex code must own rebuild, reindex, query freshness, and
  stale-projection detection.
- Workflow validator evaluation should produce machine-readable results suitable
  for the product workflow and Mission Control surfaces.
- Mission Control TUI code should consume only documented projection fields and
  keep CLI commands plus durable projections as the primary agent interface.
- Human CLI rendering should keep canonical projection logic separate from
  display text.
- Git/worktree helpers should remain convenience layers over Git, not a
  replacement sync system.
- Temporary internal migration adapters must name an owning issue, a removal
  condition, and no public CLI or Rust API compatibility promise. Mission
  closeout must inventory remaining adapter markers before the rewrite closes.

## Architecture Risks

- Some inherited Chainlink concepts and module boundaries can still obscure
  Atelier target-state work.
- Backup-style export/import can be mistaken for canonical projection/rebuild.
- SQLite state must not become the only durable source once tracked `.atelier/`
  canonical records exist.
- Process features must stay risk-scaled and configurable to avoid red tape.
