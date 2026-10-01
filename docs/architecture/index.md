# Architecture

This map covers implementation architecture for Atelier's target layered Cargo
workspace: crate ownership, persistence boundaries, local runtime state, and
inherited Chainlink structure being migrated out of the root package. Product
behavior lives in [Product](../product/index.md), product direction lives in
[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md), domain language lives in
[CONTEXT.md](../../CONTEXT.md), and fork provenance is documented in
[Chainlink Provenance](provenance.md).

## Target Workspace

Atelier is migrating to a virtual-root Cargo workspace:

- [Source Layout](source-layout.md): the target crate map for
  `atelier-core`, `atelier-workflow`, `atelier-records`, `atelier-app`, and
  `atelier-cli`, plus the migration map from the current root package.
- `crates/atelier-cli`: owns the `atelier` binary, Clap parser, terminal
  rendering, dispatch telemetry, and exit-code mapping.
- `crates/atelier-app`: owns the in-memory record index that every command
  loads from the record files, and use-case orchestration through request,
  outcome, and view-model APIs that the CLI renders.
- `crates/atelier-records`, `crates/atelier-workflow`, and
  `crates/atelier-core`: own record-file storage, workflow policy, and
  pure domain types.
- `crates/atelier-cli/tests`: migrate toward the crate that owns
  the invariant under test while preserving CLI integration coverage for
  terminal behavior.

See [Chainlink Provenance](provenance.md) for inherited module boundaries,
preservation expectations, and deferred migration areas.
See [Markdown-First Record Store](markdown-first-record-store.md) for the
record-file and domain-object boundaries that govern durable Markdown writes
(its SQLite-cache sections are superseded by ADR 0019).
See [Retention And Prune Policy](../product/retention-and-prune.md) for
user-visible cleanup classes, protection rules, Git-history recovery, and
explicit apply semantics.
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
- [ADR 0015: Missions Are Declared Workflow Policy](../adr/0015-missions-are-declared-workflow-policy.md)
- [ADR 0019: Verified Repo-Native Tracker](../adr/0019-verified-repo-native-tracker.md)
- [ADR 0018: Strategy, Draft Epics, And Adaptive Expansion](../adr/0018-strategy-draft-epics-and-adaptive-expansion.md)

## Target Architecture

[PRODUCT_INTENT.md](../../PRODUCT_INTENT.md) summarizes target architecture
principles, using the vocabulary in [CONTEXT.md](../../CONTEXT.md):

- `.atelier/` is the single project state root. It contains deterministic,
  mergeable Markdown records and tracked project config.
- `.atelier/runtime/` is the ignored local location for the canonical mutation
  lock. There is no cache: every command parses the record files it needs
  into an in-memory index.
- Command diagnostics are local-only telemetry outside record-file directories
  and do not create exported run/session records until a later contract
  explicitly opts in.
- Mutating commands write durable record files through `RecordStore`; the next
  command reads them directly.
- The root package is being deleted in favor of a virtual workspace root; all
  executable ownership moves to `crates/atelier-cli`.
- `check` reports every invalid record file and record-graph error; other
  commands fail on the first invalid record file, naming it.
- First-class concepts include missions, issues, evidence, typed links,
  workflows, workflow validators, status roles, review artifacts, and deferred run metadata; their
  user-visible behavior is defined in [Product](../product/index.md).
- Repository-owned issue workflow policy lives at the fixed
  `.atelier/workflow.yaml` path. Loading, schema validation, status categories,
  transition lookup, validator evaluation, guidance rendering, and branch
  lifecycle policy belong to `atelier-workflow`; app and CLI layers consume
  those APIs instead of maintaining parallel policy copies.

## Boundaries

- CLI parsing should stay thin and delegate behavior to command and domain
  modules.
- RecordStore code must own deterministic record-file serialization and
  record-local validation.
- The record index owns record-graph integrity checks (resolvable references,
  hierarchy shape, acyclic graphs) applied on every load.
- Workflow validator evaluation should be implemented in `atelier-workflow` and
  produce machine-readable results suitable for app orchestration, product
  workflow surfaces, and Mission Control.
- Mission Control TUI code should consume only documented projection fields and keep
  CLI commands plus durable record files as the primary agent interface.
- Human CLI rendering should keep record-file/domain loading separate from
  display text.
- Git/worktree helpers should remain convenience layers over Git, not a
  replacement sync system.

## Architecture Risks

- Some inherited Chainlink concepts and module boundaries can still obscure
  Atelier target-state work.
- Process features must stay risk-scaled and configurable to avoid red tape.
