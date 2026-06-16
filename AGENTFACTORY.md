# Agent Factory Binding

This file binds Agent Factory to this repository. Per
`docs/adr/0006-agent-guidance-ownership-boundary.md`, it is a thin repository
binding: Agent Factory owns portable orchestration discipline, while
Atelier-owned commands, workflow policy, and product docs own this
repository's tactical operator guidance.

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

## Repository Binding

- Tracker: Atelier
- Durable tracker state: committed canonical Markdown and tracked config under
  `.atelier/`
- Runtime tracker database: local `.atelier/runtime/state.db`, rebuilt from
  committed `.atelier/` records
- Other runtime/cache state under `.atelier/` is local and rebuildable; do not
  treat `.atelier/runtime/`, `.atelier/cache/`, locks, diagnostics, or local
  identity files as committed source of truth.
- Prefer the installed `atelier` command for normal tracker work. Use
  `target/debug/atelier` only when testing local CLI changes that have not been
  installed yet, and rebuild it first when it is missing or stale. Use
  `cargo run -- ...` only when a one-off rebuild plus execution is specifically
  useful.

## Orchestration Guidance

- Keep this binding small. When a rule is repository-specific and executable,
  route to an Atelier-owned surface instead of restating the full command or
  workflow contract here.
- Normal worker onboarding and recovery start with `atelier man <role>`,
  `atelier status`, `atelier issue show <id>`, and
  `atelier mission status <mission-id>`. Routine issue work starts with
  `atelier start <id>`, which owns branch preparation. Move work through issue
  transitions, durable notes, evidence records, and explicit issue or mission
  closeout surfaces rather than through hidden runtime state.
- Missions are durable active-focus records. Link executable issues to missions
  and use `atelier status` or `atelier mission status <mission-id>` to select
  worker issues from the active mission or epic graph.
- Issues are executable tracker items. When sectioned Markdown is supported,
  executable issues use `Description`, `Outcome`, `Evidence`, and optional
  `Notes`.
- Important unresolved choices become artifact-update tasks. Block dependent
  implementation on those tasks when needed.
- Validation criteria must name observable completion behavior: command output,
  rejected commands, help text, file contents, tests, lint/export checks, or
  evidence records.
- Start each slice with a stale-state preflight. If `atelier lint`,
  `atelier export --check`, `atelier status`, `atelier mission status`, or
  another normal tracker read reports invalid canonical state, stale
  projections, or unreadable tracker data, stop workflow mutation and repair
  state through Atelier-owned status, lint, doctor, and help surfaces before
  continuing.
- Use Atelier-owned proof, closeout, health, and transition surfaces through
  the destinations named in the validation router and product command docs.
- Record handoff context in durable issue notes and non-trivial proof as
  first-class evidence on the accountable issue-shaped work. Let mission status
  and audit report missing proof, missing validation/closeout work, stale
  closeout checks, and parent coverage gaps instead of restating those rules in
  this binding.
- Handoffs that include validation must say whether proof ran from the mission
  workspace, an epic branch checkout, the root checkout, or an explicitly
  isolated issue worktree, and which checkout now owns follow-up validation or
  cleanup.
- Mutating subagents should use the mission workspace by default and let
  `atelier start <id>` prepare the correct owner branch. Use an isolated issue
  worktree only when the assignment explicitly justifies extra isolation for
  contention, dirty state, high-risk work, or cross-epic separation. Worktree
  setup, branch repair, interruption recovery, and stale association handling
  belong to Atelier-owned status/help and product docs; do not edit runtime
  state by hand.
- Hidden workflow diagnostics are not normal planning, implementation, or
  closeout commands. Use them only when a binding, assignment, workflow policy,
  or closeout contract explicitly names the diagnostic command; they do not
  replace attached Outcome proof.
- Orchestrator prompts must name the model choice and a short rationale based
  on task complexity, ambiguity, risk, review depth, and proof needs.
- 5.4 Mini is suitable only when the slice is bounded, low ambiguity, and low
  risk: basic behavior validation, search, fixture repair, docs drift scans,
  command-surface inventory, transcript capture, focused tests,
  straightforward validation, stale-test inventory, or basic refactor-style
  implementation with clear owned files and objective proof.
- Do not route to 5.4 Mini when the work requires complex open-ended
  implementation, complex review, ambiguous architecture, cross-cutting
  refactors, hard debugging, security or data-loss judgment, public-contract
  redesign, or final adversarial closeout. Use a higher-reasoning model for
  those cases.
- When a Mini model is selected, the prompt must say why the scope is small
  enough, what boundaries make the work low risk, and what observable proof
  will show the assignment is complete.
- Route shell-sensitive validation commands to the repository-owned recipes in
  `docs/architecture/quality/validation.md`. Use one `cargo test` filter at a
  time or a `cargo nextest run -E` expression, quote literal `rg` patterns,
  prefer `python3`, and avoid fragile command substitution when a literal path,
  ID, or focused command is enough.
- After the first clear unrecognized command or wrong command-family error,
  stop probing neighboring names. Consult Atelier-owned help or product docs
  to recover the command surface before continuing.
- Closeout proof must quote or paraphrase the specific `Outcome` or `Evidence`
  line being proved. Broad "tests passed" summaries are not enough for
  parent-level, process-policy, or other non-trivial claims.
- Role assignment, subskill selection, model routing, and independent-review
  judgment stay in Agent Factory unless Atelier grows first-class assignment
  metadata that owns those decisions.
- Do not use command-result `--json` as the Agent Factory automation contract.
  Use focused human output, quiet acknowledgements only where natural, and
  explicit drill-down commands. Use `show`, `list`, `transition`, and `status`
  for normal planning or drill-down, not raw workflow-validator output.

## Atelier-Owned Surfaces

Use this binding to enforce the ADR 0006 ownership boundary by routing
recurring tracker behavior to Atelier-owned product surfaces rather than
restating the full command or policy contract here.

| Recurring behavior | Primary Atelier-owned destination |
| --- | --- |
| Normal command purposes, public command families, and removed-command policy | `atelier --help`, `docs/product/cli-surface.md` |
| Workflow transitions, policy diagnostics, and command-specific readiness | `.atelier/workflow.yaml`, `docs/product/workflow-configuration.md`, `atelier issue transition <id> --options` |
| Mission and work selection, blocker explanation, proof gaps, and next actions | `atelier status`, `atelier mission status [<id>]`, `atelier mission show <id>` |
| Closeout drill-down and parent-proof mapping | `atelier mission status [<id>]`, explicit validation or closeout issue evidence, `docs/architecture/quality/validation.md` |
| Evidence routing, proof placement, and independent-validation triggers | `docs/architecture/quality/validation.md`, `atelier evidence record --target issue/<id> ...` |
| Tracker health, derived-state freshness, and runtime diagnostics | `atelier lint`, `atelier doctor`, `atelier status`, `atelier mission status` |
| Onboarding and recovery signposts for normal tracker work | `atelier man <role>`, `atelier status`, `docs/product/cli-surface.md` |
| Validation command recipes and shell hygiene | `docs/architecture/quality/validation.md` |
| Validation and handoff check selection | `docs/architecture/quality/validation.md`, `atelier lint`, `atelier issue transition <id> --options`, issue Evidence sections, explicit mission validation or closeout work |

Do not preserve old command names, status aliases, output shims, or fallback
readers unless a human explicitly asks for a compatibility window. Public
compatibility decisions belong to Atelier help, product docs, workflow policy,
or validators rather than to this binding.
For Atelier, current work is derived from canonical issue status plus checkout
context. Normal workflow lives on the status, transition, note, evidence, and
closeout surfaces listed above rather than in local runtime associations.
For issue queues, `--status` means an exact workflow status (or `all`) and
`--category` means an exact derived workflow category; do not use or document
category aliases such as `in_progress`.

## Validation Routing

Use `docs/architecture/quality/validation.md`, the assigned issue Evidence
section, and explicit mission validation or closeout work to choose checks for
the current slice.
The binding may name the readiness entrypoint, but the durable check contract
belongs to Atelier-owned validation guidance, product docs, workflow policy,
and command help.
