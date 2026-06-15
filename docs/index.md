# Documentation Map

- `AGENTFACTORY.md`: thin Agent Factory binding for this repository; per
  [ADR 0006](adr/0006-agent-guidance-ownership-boundary.md), it routes
  repo-specific tactical guidance to Atelier-owned surfaces.
- `AGENTS.md`: concise entry point for agents working in this repository.
- [CONTEXT.md](../CONTEXT.md): domain language and resolved model choices.
- [SPEC.md](../SPEC.md): product intent and target behavior for Atelier.
- [docs/spec/storage/export/rebuild/canonical-layout.md](spec/storage/export/rebuild/canonical-layout.md):
  canonical `.atelier/` record file, ignored runtime/cache, compatibility, and
  rebuild layout.
- [docs/spec/bulk-plan/schema.md](spec/bulk-plan/schema.md):
  versioned JSON contract, validation diagnostics, dry-run preview shape, and
  fixtures for authored bulk graph plans.
- [docs/spec/agent-factory/tracker-replacement-mvp.md](spec/agent-factory/tracker-replacement-mvp.md):
  minimum human command, storage, and workflow contract required before Atelier
  replaces Beads for this repository and Agent Factory.
- `docs/adr/`: durable architecture and process choices.
- [docs/adr/0001-project-scoped-random-record-ids.md](adr/0001-project-scoped-random-record-ids.md):
  accepted choice that canonical record IDs are project-scoped random IDs
  such as `atelier-z1p8`, not typed numeric IDs or semantic slugs.
- [docs/adr/0002-markdown-first-record-store.md](adr/0002-markdown-first-record-store.md):
  accepted choice that Markdown records are canonical, SQLite is a
  rebuildable projection index plus local runtime state, and export is a
  compatibility/repair path during migration.
- [docs/product/index.md](product/index.md): product behavior, workflow model,
  public command surface, and human interface contracts.
- [docs/product/development-setup.md](product/development-setup.md): required
  tools, Rust toolchain, `cargo-nextest`, local environment/secrets policy, and
  devcontainer decision.
- [docs/product/repository-contribution.md](product/repository-contribution.md):
  repository contribution entry points, GitHub collaboration-file
  classification, and tracker-first ownership policy.
- [docs/product/zen.md](product/zen.md): product principles for
  semi-autonomous mission work, evidence, validation, and closeout.
- [docs/architecture/index.md](architecture/index.md): implementation
  architecture, target Cargo workspace, and ownership map.
- [docs/architecture/source-layout.md](architecture/source-layout.md):
  target crate responsibilities, dependency direction, internal API policy,
  temporary adapter limits, and current-file orientation during the migration.
- [docs/architecture/provenance.md](architecture/provenance.md): Chainlink
  provenance, inherited module boundaries, preservation expectations, and
  deferred migration areas.
- [docs/product/work-model.md](product/work-model.md): mission,
  milestone, epic, issue, workflow validator, and evidence relationships.
- [docs/product/milestone-records.md](product/milestone-records.md):
  first-class checkpoint record fields, validation model, evidence links, and
  command-surface ownership.
- [docs/product/workflow-configuration.md](product/workflow-configuration.md):
  fixed `.atelier/workflow.yaml` issue-policy path, schema, built-in validators,
  guidance templates, strict errors, and starter workflow examples.
- [docs/architecture/markdown-first-record-store.md](architecture/markdown-first-record-store.md):
  RecordStore, ProjectionIndex, and RuntimeState boundaries for Markdown-first
  durable writes, issue activity sidecars, rebuildable SQLite indexes, and
  local runtime data.
- [docs/architecture/local-command-diagnostics.md](architecture/local-command-diagnostics.md):
  local command telemetry storage, redaction defaults, opt-out controls,
  retention behavior, event fields, and Mission Control export boundary.
- [docs/product/mission-control-tui.md](product/mission-control-tui.md):
  Mission Control TUI projection dependencies, degradation rules, navigation
  model, mutation boundary, and fixture expectations.
- [docs/product/cli-surface.md](product/cli-surface.md): public CLI
  core command surface, removed compatibility commands, and inherited utility
  disposition.
- [docs/product/command-audit/index.md](product/command-audit/index.md):
  role ownership, naming, documentation, output hierarchy, and role-specific
  guide proposal for each root command surface.
- [docs/product/human-cli-output.md](product/human-cli-output.md):
  human-readable CLI output grammar, formatter boundaries, color/width policy,
  and test expectations.
- `docs/architecture/quality/index.md`: quality documentation index.
- `docs/architecture/quality/architecture-quality.md`: vocabulary for
  architecture review.
- `docs/architecture/quality/standards.md`: code and documentation standards.
- `docs/architecture/quality/validation.md`: validation router and result
  states.
- [docs/architecture/quality/mission-log-insights.md](architecture/quality/mission-log-insights.md):
  findings from mining recent long-running Codex mission transcripts for docs,
  Agent Factory, and CLI improvement opportunities.
- `docs/references/`: notes on external projects and prior art that inform
  Atelier design.
