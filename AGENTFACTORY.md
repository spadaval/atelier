# Agent Factory Binding

This file binds Agent Factory to this repository.

## Sources

- Agent instructions: `AGENTS.md`
- Docs map: `docs/index.md`
- Domain context: `CONTEXT.md`
- Product intent: `SPEC.md`
- Product index: `docs/product/index.md`
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
- Durable tracker state: committed canonical Markdown and tracked config under
  `.atelier/`
- Runtime tracker database: local `.atelier/state.db`, rebuilt from
  committed `.atelier/` records
- Missions are durable active-focus records. Link executable issues to missions
  and select worker issues from the active mission or epic graph.
- Issues are executable tracker items.
- Important unresolved choices become artifact-update tasks. Block dependent
  implementation on those tasks when needed.
- Validation criteria must name observable completion behavior: command output,
  rejected commands, help text, file contents, tests, lint/export checks, or
  evidence records.
- Use the validation router for proof routing:
  `docs/architecture/quality/validation.md`. Ordinary executable issues prove
  their own scoped Outcome on the issue. Risky, broad, public-contract,
  migration, cross-cutting, Agent Factory process, stale-test, docs/help parity,
  and parent closeout claims require first-class evidence and may require a
  separate validation or closeout issue.
- Use durable issue notes for handoff context, caveats, skipped optional checks,
  or trivial docs-only proof. Use first-class evidence for non-trivial proof,
  workflow validation, process-policy changes, failed or deferred checks, and
  any result a future worker must inspect. Use separate validation issues when
  the implementer should not validate their own claim.
- Prefer the installed `atelier` command for normal tracker work. Use
  `target/debug/atelier` only when testing local CLI changes that have not been
  installed yet, and rebuild it first when it is missing or stale. Use
  `cargo run -- ...` only when a one-off rebuild plus execution is specifically
  useful.
- Mission:
  - `atelier status`
  - `atelier mission list`
  - `atelier mission show <id>`
  - `atelier mission status [<id>]`
  - `atelier mission create "..."`
  - `atelier mission update <id> --status <draft|ready|active|closed>`
  - `atelier mission add-work <mission-id> <issue-id>`
- Work/evidence:
  - `atelier worktree for <issue-id>`
  - `atelier start <issue-id>`
  - `atelier status`
  - `atelier finish [issue-id]`
  - `atelier evidence add --kind <kind> --result <result> "summary"`
  - `atelier evidence attach <evidence-id> issue <issue-id>`
  - `atelier workflow validate issue <issue-id>`
  - `atelier workflow validate mission <mission-id>`
- Issues:
  - `atelier issue list --ready`
  - `atelier issue list --blocked`
  - `atelier issue list --status open`
  - `atelier issue show <id>`
  - `atelier issue transition <id> --options`
  - `atelier mission show <id>`
  - `atelier issue update <id> --claim`
  - `atelier issue update <id> --append-notes "..."`
  - `atelier issue close <id> --reason "..."`
- Do not use command-result `--json` as the Agent Factory automation contract.
  Use focused human output, quiet acknowledgements only where natural, and
  explicit drill-down commands.
- Sync/state:
  - `git pull`
  - `atelier export --check`
  - `atelier export`
  - `git status --short --branch`
  - `git push`
- Health:
  - `atelier lint`
  - `atelier lint <id>`
  - `atelier doctor`
- Mission completion requires all linked work closed, required evidence
  attached, workflow validators passing, and a clean Git worktree.
- Do not preserve old command names, status aliases, output shims, or fallback
  readers unless a human explicitly asks for a compatibility window.

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
