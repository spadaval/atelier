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
- Issues are executable tracker items. When sectioned Markdown is supported,
  executable issues use `Description`, `Outcome`, `Evidence`, and optional
  `Notes`.
- Important unresolved choices become artifact-update tasks. Block dependent
  implementation on those tasks when needed.
- Validation criteria must name observable completion behavior: command output,
  rejected commands, help text, file contents, tests, lint/export checks, or
  evidence records.
- Use the validation router for proof routing:
  `docs/architecture/quality/validation.md`. Ordinary executable issues prove
  their own scoped Outcome on the issue. Risky, broad, public-contract,
  process-policy, migration, cross-cutting, Agent Factory process, stale-test,
  docs/help parity, parent-level, epic, and mission claims require first-class
  evidence and may require a separate validation or closeout issue.
- Use durable issue notes for handoff context, caveats, skipped optional checks,
  or trivial docs-only proof. Use first-class evidence for non-trivial proof,
  advanced policy checks, process-policy changes, failed or deferred checks,
  and any result a future worker must inspect. Use separate validation issues
  when the implementer should not validate their own claim.
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
  - `atelier mission audit <id>`
  - `atelier mission create "..."`
  - `atelier mission update <id> --status <draft|ready|active|closed>`
  - `atelier mission add-work <mission-id> <issue-id>`
- Work/evidence:
  - `atelier worktree for <issue-id>`
  - Mutating subagents should use isolated issue worktrees unless the
    assignment explicitly explains why a shared checkout is safer.
  - If setup or removal is interrupted and the recorded worktree path no
    longer exists, use `atelier worktree repair <issue-id>` to clear the stale
    local runtime association instead of editing runtime state by hand.
  - `atelier start <issue-id>`
  - `atelier status`
  - `atelier finish [issue-id]`
  - `atelier evidence record --target issue/<issue-id> --kind <kind> --result <result> "summary"`
  - `atelier evidence record --target issue/<issue-id> --kind <kind> --result <result> -- <command>`
  - `atelier evidence attach <evidence-id> issue <issue-id>`
- Advanced policy diagnostics:
  - Hidden workflow diagnostics are not normal planning, implementation, or
    closeout commands. Use them only when a binding, assignment, workflow policy,
    or closeout contract explicitly names the diagnostic command; they do not
    replace attached Outcome proof.
- Model routing:
  - Orchestrator prompts must name the model choice and a short rationale based
    on task complexity, ambiguity, risk, review depth, and proof needs.
  - 5.4 Mini is suitable only when the slice is bounded, low ambiguity, and
    low risk: basic behavior validation, search, fixture repair, docs drift
    scans, transcript capture, focused tests, straightforward validation,
    stale-test inventory, or basic refactor-style implementation with clear
    owned files and objective proof.
  - Do not route to 5.4 Mini when the work requires complex open-ended
    implementation, complex review, ambiguous architecture, cross-cutting
    refactors, hard debugging, security or data-loss judgment, public-contract
    redesign, or final adversarial closeout. Use a higher-reasoning model for
    those cases.
  - When a Mini model is selected, the prompt must say why the scope is small
    enough, what boundaries make the work low risk, and what observable proof
    will show the assignment is complete.
- Issues:
  - `atelier issue list --ready`
  - `atelier issue list --blocked`
  - `atelier issue list --status open`
  - `atelier issue show <id>`
  - `atelier issue transition <id> --options`
  - `atelier mission show <id>`
  - `atelier issue update <id> --claim`
  - `atelier search <query>`
  - `atelier link add issue <id> issue <related-id> --type <type>`
  - `atelier link remove issue <id> issue <related-id> --type <type>`
  - `atelier link list issue <id>`
  - `atelier graph impact <id>`
  - `atelier graph tree --compact`
  - `atelier note add issue <id> "..."`
  - `atelier maintenance delete issue <id> --force`
  - `atelier issue update <id> --append-notes "..."`
  - `atelier issue close <id> --reason "..."`
- Do not use command-result `--json` as the Agent Factory automation contract.
  Use focused human output, quiet acknowledgements only where natural, and
  explicit drill-down commands. Use `show`, `list`, `transition`, and `status`
  for normal planning or drill-down, not raw workflow-validator output.
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
  attached, required advanced policy checks passing, `atelier mission audit
  <id>` mapping mission validation expectations and linked epic outcomes to
  work and evidence, and a clean Git worktree.
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
