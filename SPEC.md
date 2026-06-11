# Atelier Specification

Atelier is a local-first, agent-native work tracker for complex software missions.
It starts from the Chainlink codebase, but the intended product is not a thin
rename. The goal is to combine Chainlink's SQLite-backed operational machinery
with Braid's repo-native simplicity and then add the workflow, evidence, and
mission-control features needed by agent-factory style orchestration.

## Core Thesis

Atelier should use:

- Markdown records for canonical state at rest.
- SQLite for rebuildable projection indexes and local runtime state.
- Git for merge, review, and long-term audit.
- Agent-facing commands as the primary interface.
- Optional UI surfaces built on top of mechanical projections.

Markdown records are the durable, mergeable repo surface. The SQLite database is
the fast local projection and runtime store. It supports queries, locks,
sessions, workflow checks, and Mission Control projections without making
Markdown parsing the hot path for every command. A worktree should be able to
rebuild its local SQLite projection from committed Markdown records after
checkout, pull, merge, or clone.

## Starting Point

This repository begins as a copy of `dollspace-gay/chainlink`.

Chainlink provides useful foundations:

- Rust CLI structure.
- SQLite database and migrations.
- Issue CRUD.
- Parent/subissue relationships.
- Labels, comments, relations, and dependencies.
- Milestones.
- Sessions and handoff notes.
- Agent identity.
- Lock and sync concepts.
- Agent identity and local runtime concepts.
- Hooks, rules, and context-provider direction.
- JSON handling in durable files, authored inputs, and diagnostics.

Braid provides important product-shape ideas:

- Repo-legible issue state.
- Simple command-oriented workflow.
- Git/worktree ergonomics.
- Agent worktree creation.
- Ready/start/done flow.
- Human-readable markdown records.
- Lightweight mental model.

Atelier should preserve Braid's elegance without making markdown parsing the hot
path for every command.

## Non-Goals

Atelier should not become:

- A SaaS-backed issue tracker.
- A Jira clone.
- A Dolt-backed database system.
- A mandatory red-tape engine.
- A TUI-first product.
- A system where every issue maps rigidly to one agent session.

Interactive UI can come later. The foundation should be a crisp human-first CLI
with durable file projections.

## Storage Model

Atelier should implement this storage contract:

```text
.atelier/
  state.db              # live local SQLite state
  agent.json            # machine-local agent identity
  cache/                # derived local-only data

.atelier-state/
  issues/
    atelier-z1p8.md
  missions/
    atelier-k7mq.md
  milestones/
    atelier-4x9t.md
  plans/
    atelier-p3v6.md
  evidence/
    atelier-n8da.md
  mission-control.json  # derived projection, not rebuild source
```

The exact layout can change, but the principles should not:

- Canonical Markdown records must be deterministic.
- Canonical Markdown records must be sufficient to rebuild SQLite projections.
- `export --check` must detect stale canonical records and derived projections.
- Mutating commands should write canonical Markdown records first, then refresh
  or mark stale the SQLite projection.
- Git merges should happen through Markdown record files, not by merging SQLite
  files.
- SQLite should be rebuildable after checkout, merge, pull, or clone.

The existing Chainlink export/import system is backup-oriented. Atelier needs a
Markdown-first canonical record store with rebuildable projections instead.

## Record Identity

Atelier uses one canonical, human-facing ID for every durable record:

```text
<project-slug>-<random-base36>
```

For this repository, examples are `atelier-z1p8`, `atelier-k7mq`, and
`atelier-4x9t`. Record kind is metadata, not part of identity, so issues,
missions, milestones, plans, and evidence share the same ID shape. Titles are
mutable text and must never be identity.

This replaces typed numeric target IDs such as `ISS-0001` or `MIS-0001` and
separate shorthand IDs such as `#53`. The cutover should migrate existing
records to the new ID form directly; Atelier should not maintain a legacy
dual-ID implementation.

## Domain Model

Atelier should distinguish durable work concepts rather than forcing everything
to be an issue.

### Mission

A mission is a high-level objective that may span hours or days. It is useful
for orchestrators and Mission Control. It should contain intent, constraints,
active milestones, linked plans, status, validation expectations, and current
health.

Missions should be focused on the goal or end state, not on a specific task.
They are the right shape when the objective is large enough to require at least
one epic of accountable work beneath it. Smaller work should stay as an issue,
while mission execution should be split into epics, tasks, validation, review,
documentation, or closeout issues linked back to the mission.

### Milestone

A milestone is a validated intermediate checkpoint state. It is not a work
container or super-epic, and it is not merely a vague point on a roadmap. A
milestone should define:

- Desired state.
- Scope boundaries.
- Validation criteria.
- Linked work.
- Required or accepted evidence.
- Completion state.

Epics and tasks may contribute to a milestone, and evidence may validate a
milestone's criteria. The milestone itself should remain a target-state record.

### Issue

An issue is a durable accountability unit. It may be small enough for one agent
run, but the system must not require one issue to equal one run. Issues can be
tasks, bugs, research items, implementation slices, review items, validation
items, or custom configured types.

### Plan

A plan records intended execution when that intent matters beyond ephemeral
context. Plans should be stored when they create scope, coordinate multiple
agents, define sequencing, explain decisions, or survive across sessions. Scratch
plans can remain ephemeral.

Plans must be adaptable. The tool should make plan drift visible and allow plans
to be revised with reasons as new facts emerge.

### Evidence

Evidence records prove that validation happened. Evidence may point to test
results, logs, screenshots, videos, API responses, benchmark output, review
notes, or generated reports.

Evidence metadata should include:

- ID.
- Linked issue, milestone, mission, workflow validator result, or other
  evidence target.
- Kind.
- Result.
- Summary.
- Path or URI.
- Size.
- Hash when applicable.
- Created time.
- Producer or validator.

Small artifacts may live in the repo. Large artifacts should use external
storage while preserving metadata in repo state. The first supported backend is
metadata-only evidence records with optional repository-relative paths or
external URIs; payload copying, hashing, upload, retention, and garbage
collection are deferred until a dedicated artifact-backend contract lands.

### Run

A run/session/slice is execution metadata, not the primary unit of work. A run
may touch one issue, part of one issue, or several issues. Runs should record
what happened mechanically enough for Mission Control without forcing agents
into rigid accounting.

Local command diagnostics are not run records. Command duration, exit status,
redacted command family, workspace grouping, and phase timing events may be
recorded in a user-local diagnostics store for performance analysis, but they
must not be exported to `.atelier-state/` or treated as Mission Control run
projection data until an explicit run/session projection contract exists.

## Relationships

Relationships should be explicit without exposing a generic graph-editing
workflow as the normal product model. Canonical Markdown stores relationships in
one `relationships` section with four buckets:

- `blocks` for operational issue readiness blockers.
- `children` for hierarchy, scope, and mission work.
- `attachments` for owned supporting records such as plans and evidence.
- `relates` for peer semantic relationships such as related, duplicates,
  supersedes, derived_from, and implements.

Domain commands own relationship mutations. For example, issue blocking,
mission work, evidence attachment, and plan linkage should be surfaced through
their domain command groups instead of a public generic `link` command.

## Workflows

Atelier should support configurable workflows. A workflow defines allowed phases,
transitions, required fields, workflow validators, evidence requirements, and
closure rules.

Example workflow:

```yaml
types:
  epic:
    workflow: epic_delivery

workflows:
  epic_delivery:
    phases:
      - research
      - impact_report
      - planning
      - implementation
      - code_review
      - validation
      - done
    transitions:
      research: [impact_report]
      impact_report: [planning, research]
      planning: [implementation, research]
      implementation: [code_review, validation]
      code_review: [implementation, validation]
      validation: [done, implementation]
    done_requires:
      evidence:
        min_count: 1
      validators:
        - tests_passed
        - durable_state_current
```

Workflows should scale with risk. Small tasks should not require heavyweight
ceremony unless policy says so.

## Rules, Lint, And Guidance

Atelier should support three layers of process:

- Mechanical rules: enforceable by the CLI.
- Lint rules: warnings or errors over tracker state.
- Advisory guidance: surfaced at relevant actions and optionally reviewed by an
  LLM.

Examples:

- Required evidence before `done`.
- Required plan before implementation for large missions.
- Warning when a summary is too long or too vague.
- Warning when a milestone lacks validation criteria.
- Error when durable records or derived projections are stale.
- Error when a workflow transition is invalid.
- Warning when implementation starts on `main`.

Fuzzy guidance should live close to the action it affects. Agents should receive
the relevant rule at the point of use, not a wall of generic process text.

## Avoiding Red Tape

Atelier must make the right thing easy without drowning agents in process.

Process should be:

- Configurable.
- Risk-scaled.
- Overrideable with a recorded reason.
- Split between start requirements and close requirements.
- Strict only where strictness protects correctness or coordination.

Rules should have severities:

- `info`
- `warn`
- `error`
- `policy`

Waivers should be explicit and visible in Mission Control.

## Branches And Worktrees

Atelier should preserve Braid's agent worktree ergonomics.

Desired commands:

```text
atelier agent init <name>
atelier work start atelier-z1p8
atelier worktree for atelier-z1p8
atelier work finish atelier-z1p8
atelier worktree merge
```

The default branch model should be opinionated but configurable:

```text
main
  integration branch

work/atelier-z1p8-short-slug
  normal issue implementation branch

mission/atelier-k7mq-short-slug
  optional coordinated mission branch

agent/<agent-id>
  optional long-lived agent branch/worktree
```

Useful enforcement:

- Warn or fail when implementation starts on `main`.
- Refuse claim when the worktree is dirty unless overridden.
- Record branch/worktree association.
- Refuse `done` when durable records or derived projections are stale.
- Allow multi-issue slices with explicit intent.

The worktree feature is a convenience layer over Git, not a replacement sync
system.

Normal tracked work uses explicit work association rather than inherited
Chainlink lock sync. The default workflow is Git branch/worktree state plus
local Atelier runtime association and canonical export freshness checks.

## Validation And Workflow Validators

Evidence should be a first-class condition for closing work. Workflow validators
evaluate whether a record can advance or close.

Example validators:

- `durable_state_current`
- `tests_passed`
- `review_complete`
- `evidence_attached`
- `validation_criteria_satisfied`
- `no_blocking_lints`

Validators should produce machine-readable results for Mission Control.

## Mission Control

Atelier should maintain enough mechanical state to drive a dashboard or local UI.

Mission Control should be able to show:

- Active missions.
- Milestone progress.
- Open blockers.
- Active agents and runs.
- Branch/worktree associations.
- Claims and locks.
- Stale exports.
- Required evidence.
- Workflow validator failures.
- Plan drift.
- Recent decisions.
- Items ready for review or validation.

The first Mission Control slice should be CLI-native: `atelier mission status
[<id>]` should summarize mission health, blockers, evidence gaps, validator
failures, closeout readiness, and next actions for agents and orchestrators.
Deterministic JSON projections and richer UI surfaces can follow once the CLI
status contract proves the needed state model.

## Command Philosophy

The CLI should be small, composable, and agent-friendly.

Representative commands:

```text
atelier init
atelier issue next
atelier issue list --ready
atelier issue show atelier-z1p8
atelier issue create
atelier mission create
atelier mission status
atelier milestone create
atelier plan create
atelier mission add-work
atelier work start
atelier work finish
atelier evidence add
atelier workflow validate
atelier lint
atelier export
atelier export --check
atelier rebuild
atelier doctor
```

Every command that agents call should provide focused human-readable output with
the actionable identifiers, state, and next commands needed for the immediate
workflow. Durable Markdown records and explicit projection files are the
machine-readable source of truth, not command-result JSON.

## Initial Milestones

### Milestone 1: Establish The Fork

- Copy Chainlink into the new project.
- Rename package, binary, directories, and user-facing text.
- Preserve tests where practical.
- Document provenance and architectural intent.

### Milestone 2: Canonical Export/Rebuild

- Define `.atelier-state`.
- Export deterministic per-record files.
- Add `export --check`.
- Add `rebuild`.
- Make SQLite rebuildable from committed Markdown records.
- Keep `.atelier/state.db` ignored as local runtime state; committed durable
  state lives under `.atelier-state/`.

### Milestone 3: Braid-Style Worktrees

- Add agent worktree creation.
- Add work branch helpers.
- Associate claims with branches/worktrees.
- Rebuild SQLite in new worktrees.

### Milestone 4: Domain Model Upgrade

- Add first-class missions, milestone checkpoint records, plans, evidence,
  workflow validators, and runs.
- Add canonical relationship buckets and domain-specific relationship commands.
- Keep compatibility migration paths where reasonable.

### Milestone 5: Workflows And Rules

- Add configurable types and workflows.
- Add validator-backed transitions.
- Add linter severities and waivers.
- Surface action-aware guidance.

### Milestone 6: Mission Control Projection

- Add JSON projections for active missions, agents, blockers, workflow
  validator failures, evidence, branches, and plan drift.
- Defer rich UI until projections are useful.

## Open Questions

- Resolved for Milestone 1: the canonical binary is `atelier`; short aliases
  such as `atl` are deferred until the install story is stable.
- Resolved for Milestone 1: `.atelier/state.db` is ignored local runtime state,
  while `.atelier-state/` is the committed rebuild source.
- Should sessions be exported, partially exported, or treated as local runtime
  metadata? Command diagnostics are resolved separately as local-only telemetry
  and do not answer this broader run/session policy.
- Which external artifact backend should be implemented after metadata-only
  path/URI evidence records?
- What shared or remote lock policy is needed after work association and
  Mission Control projections stabilize?
- What should the default workflow be for tiny tasks?
