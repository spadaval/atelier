# Documentation Map

- `AGENTFACTORY.md`: agent-factory bindings for this repository.
- `AGENTS.md`: concise entry point for agents working in this repository.
- [CONTEXT.md](../CONTEXT.md): domain language and ambiguity decisions.
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
- `docs/adr/`: durable architecture and process decisions.
- [docs/adr/0001-project-scoped-random-record-ids.md](adr/0001-project-scoped-random-record-ids.md):
  accepted decision that canonical record IDs are project-scoped random IDs
  such as `atelier-z1p8`, not typed numeric IDs or semantic slugs.
- [docs/adr/0002-markdown-first-record-store.md](adr/0002-markdown-first-record-store.md):
  accepted decision that Markdown records are canonical, SQLite is a
  rebuildable projection index plus local runtime state, and export is a
  compatibility/repair path during migration.
- [docs/architecture/index.md](architecture/index.md): architecture and
  ownership map.
- [docs/architecture/provenance.md](architecture/provenance.md): Chainlink
  provenance, inherited module boundaries, preservation expectations, and
  deferred migration areas.
- [docs/architecture/work-model.md](architecture/work-model.md): mission,
  milestone, epic, issue, workflow validator, and evidence relationships.
- [docs/architecture/milestone-records.md](architecture/milestone-records.md):
  first-class checkpoint record fields, validation model, evidence links, and
  command-surface ownership.
- [docs/architecture/workflow-configuration.md](architecture/workflow-configuration.md):
  repository-owned workflow policy path, schema, validators, hooks, reload
  behavior, and risk-scaled examples.
- [docs/architecture/markdown-first-record-store.md](architecture/markdown-first-record-store.md):
  RecordStore, ProjectionIndex, and RuntimeState boundaries for Markdown-first
  durable writes, issue activity sidecars, rebuildable SQLite indexes, and
  local runtime data.
- [docs/architecture/local-command-diagnostics.md](architecture/local-command-diagnostics.md):
  local command telemetry storage, redaction defaults, opt-out controls,
  retention behavior, event fields, and Mission Control export boundary.
- [docs/architecture/mission-control-tui.md](architecture/mission-control-tui.md):
  Mission Control TUI projection dependencies, degradation rules, navigation
  model, mutation boundary, and fixture expectations.
- [docs/architecture/cli-surface.md](architecture/cli-surface.md): public CLI
  core command surface, removed compatibility commands, and inherited utility
  disposition.
- [docs/architecture/human-cli-output.md](architecture/human-cli-output.md):
  human-readable CLI output grammar, formatter boundaries, color/width policy,
  and test expectations.
- `docs/architecture/quality/index.md`: quality documentation index.
- `docs/architecture/quality/architecture-quality.md`: vocabulary for
  architecture review.
- `docs/architecture/quality/standards.md`: code and documentation standards.
- `docs/architecture/quality/validation.md`: validation router and result
  states.
- `docs/references/`: notes on external projects and prior art that inform
  Atelier design.
