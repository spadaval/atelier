# Agent Factory Binding

This file binds the generic agent-factory operating model to this repository's
concrete files, commands, and product-specific skills.

## Sources

- Agent instructions: `AGENTS.md`
- Docs map: `docs/index.md`
- Domain context: `CONTEXT.md`
- Product intent: `SPEC.md`
- ADR directory: `docs/adr/`
- Architecture index: `docs/architecture/index.md`
- Quality index: `docs/architecture/quality/index.md`
- Architecture quality vocabulary: `docs/architecture/quality/architecture-quality.md`
- Code standards: `docs/architecture/quality/standards.md`
- Validation router: `docs/architecture/quality/validation.md`
- Tracker replacement MVP:
  `docs/spec/agent-factory/tracker-replacement-mvp.md`

## Tracker

- Tracker: Atelier
- Durable tracker state: committed `.atelier-state/`
- Runtime tracker database: local `.atelier/state.db`, rebuilt from
  `.atelier-state/`
- Missions are durable goal records for product outcomes and coordinated
  workstreams. Link executable issues to missions and keep mission status in
  sync with linked work during orchestration and closeout.
- Issues are executable tracker items. Use them for claimable implementation,
  validation, documentation, migration, audit, and readiness slices.
- Mission commands:
  - `atelier mission list`
  - `atelier mission show <id>`
  - `atelier mission create "..."`
  - `atelier mission update <id> --status <open|closed>`
  - `atelier link add mission <mission-id> issue <issue-id> --type advances`
  - `atelier workflow validate mission <id>`
- Normal tracker commands:
  - `atelier issue ready`
  - `atelier issue list --status open`
  - `atelier issue show <id>`
  - `atelier mission show <id>`
  - `atelier issue update <id> --claim`
  - `atelier issue update <id> --append-notes "..."`
  - `atelier issue close <id> --reason "..."`
- Human-first workflow: do not use command-result `--json` as the Agent Factory
  automation contract. Inspect records with focused human output, use quiet
  output only for commands that naturally acknowledge one ID/count/status, and
  run explicit drill-down commands such as `atelier issue show <id>`,
  `atelier mission show <id>`, `atelier issue ready`,
  `atelier workflow validate mission <id>`, `atelier export --check`,
  `atelier lint`, and `atelier doctor` when more state is needed.
- Sync and state commands:
  - `git pull`
  - `atelier rebuild`
  - `atelier export --check`
  - `atelier export`
  - `git status --short --branch`
  - `git push`
- Tracker health commands:
  - `atelier lint`
  - `atelier lint <id>`
  - `atelier doctor`
- Closeout expectation: a mission is not done merely because the current
  issue is closed. Confirm linked work, blockers, validation, and mission
  status with `atelier mission show <id>` before declaring the mission complete.
- Beads commands are not part of the Agent Factory path in this repository.
  `atelier import-beads` remains only a one-way external import command.

## Checks

- Markdown formatting: `git diff --check -- '*.md'`
- Rust formatting: `cargo fmt -- --check`
- Diff whitespace: `git diff --check`
- Rust test suite: `cargo nextest run`
- Extended property tests:
  `cargo nextest run --profile extended --run-ignored=only`
- Tracker export freshness: `atelier export --check`
- Tracker lint: `atelier lint`
- Tracker health: `atelier doctor`
- Cargo compatibility check: `cargo test`

## Product-Specific Skills

- None yet. Add repository-local skills under `.agents/skills/` and bind them
  here when repeated Atelier-specific workflows emerge.
