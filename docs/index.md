# Documentation Map

- `.agents/skills/agent-factory/SKILL.md`: repository-owned Agent Factory skill
  source and subskill references used while Atelier's workflow is still being
  refined.
- `AGENTS.md`: concise entry point and repository reference map for agents
  working in this repository.
- [CONTEXT.md](../CONTEXT.md): domain language and resolved model choices.
- [PRODUCT_INTENT.md](../PRODUCT_INTENT.md): product direction, principles, and
  target-state constraints for Atelier.
- [docs/spec/storage/export/rebuild/canonical-layout.md](spec/storage/export/rebuild/canonical-layout.md):
  canonical `.atelier/` record file, ignored projection/cache, compatibility, and
  rebuild layout.
- [docs/spec/bundle/schema.md](spec/bundle/schema.md):
  versioned JSON contract, validation diagnostics, preview output shape, and
  fixtures for authored bundle graph deltas.
- [docs/spec/agent-factory/tracker-replacement-mvp.md](spec/agent-factory/tracker-replacement-mvp.md):
  minimum human command, storage, and workflow contract required before Atelier
  replaces Beads for this repository and Agent Factory.
- `docs/adr/`: durable architecture and process choices.
- [docs/adr/0001-project-scoped-random-record-ids.md](adr/0001-project-scoped-random-record-ids.md):
  accepted choice that canonical record IDs are project-scoped random IDs
  such as `atelier-z1p8`, not typed numeric IDs or semantic slugs.
- [docs/adr/0002-markdown-first-record-store.md](adr/0002-markdown-first-record-store.md):
  accepted choice that Markdown records are canonical, SQLite is a
  rebuildable projection index, and export is a
  hidden/admin compatibility and determinism-check path during migration.
- [docs/adr/0011-native-review-modes-and-room-authority.md](adr/0011-native-review-modes-and-room-authority.md):
  accepted choice that projects configure exactly one review mode, native
  rooms live under `.atelier/reviews/`, the public command surface is
  `atelier review`, and review merge does not transition issue workflow.
- [docs/adr/0013-workflow-transition-actions-and-branching.md](adr/0013-workflow-transition-actions-and-branching.md):
  accepted choice that transition actions replace effects, branch mutation is
  declared transition work, and v1 rejects generic capabilities, separate
  branch lifecycles, mission-only branching, and arbitrary hooks.
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
  architecture and ownership map.
- [docs/architecture/provenance.md](architecture/provenance.md): Chainlink
  provenance, inherited module boundaries, preservation expectations, and
  deferred migration areas.
- [docs/product/work-model.md](product/work-model.md): mission, deferred
  checkpoint semantics, epic, issue, workflow validator, and evidence
  relationships.
- [docs/product/validation.md](product/validation.md): product language for
  outcomes, proof, evidence, validation, closeout, and proof visibility in
  normal operator surfaces.
- [docs/product/milestone-records.md](product/milestone-records.md): deferred
  checkpoint semantics and the v1 rule that validation data stays on issues,
  epics, missions, and evidence.
- [docs/product/workflow-configuration.md](product/workflow-configuration.md):
  fixed `.atelier/workflow.yaml` issue-policy path, schema, built-in validators,
  review field and review room records, strict errors, and starter workflow
  examples.
- [docs/product/retention-and-prune.md](product/retention-and-prune.md):
  cleanup classes, protection rules, Git-history recovery, and explicit apply
  semantics.
- [docs/architecture/markdown-first-record-store.md](architecture/markdown-first-record-store.md):
  RecordStore and ProjectionIndex boundaries for Markdown-first durable writes,
  issue activity sidecars, rebuildable SQLite indexes, and local cache data.
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
