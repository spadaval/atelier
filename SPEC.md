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
  config.toml           # tracked project tracker configuration
  issues/
    atelier-z1p8.md
    atelier-z1p8.activity/
      20260611T160930000000Z.md
  missions/
    atelier-k7mq.md
  milestones/
    atelier-4x9t.md
  plans/
    atelier-p3v6.md
  evidence/
    atelier-n8da.md
  runtime/              # ignored local runtime state
    state.db
    agent.json
    locks/
    diagnostics/
  cache/                # ignored derived local-only data
```

The exact layout can change, but the principles should not:

- Canonical Markdown records must be deterministic.
- Canonical Markdown records must be sufficient to rebuild SQLite projections.
- `export --check` must detect stale canonical records and derived projections
  during the compatibility window, and the target lint/rebuild checks must
  validate canonical `.atelier/` Markdown directly.
- Mutating commands should write canonical Markdown records first, then refresh
  or mark stale the SQLite projection.
- Git merges should happen through Markdown record files, not by merging SQLite
  files.
- SQLite should be rebuildable after checkout, merge, pull, or clone.
- Runtime and cache paths under `.atelier/runtime/` and `.atelier/cache/` are
  ignored local state and are never the durable project record source.

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

Evidence records prove that accountable work, review, validation, or closeout
happened. Evidence may point to test results, logs, screenshots, videos, API
responses, benchmark output, review notes, or generated reports. Normal evidence
targets issue-shaped work because issues own accountability; mission,
milestone, and epic readiness is derived from linked implementation,
validation, review, and closeout work rather than direct parent attachments.

Evidence metadata should include:

- ID.
- Accountable target IDs, normally issue-shaped work such as implementation,
  review, validation, or closeout issues.
- Proof scope: the issue Outcome line, parent validation criterion, workflow
  validator result, milestone criterion, or review/audit claim being proven.
- Kind.
- Result.
- Summary.
- Commands, command exit status, bounded stdout/stderr summaries, artifacts,
  paths, or URIs.
- Agent identity or producer.
- Independence level, such as implementer-produced, peer validation,
  independent validation, closeout audit, or adversarial validation.
- Residual risks.
- Follow-up issue IDs for defects, deferred proof, or remaining work.
- Size.
- Hash when applicable.
- Created time.

Direct mission evidence links are retained only for legacy imports, migration
notes, or explicit closeout mirroring. New mission proof should be captured on
the linked closeout or validation issue that performed the audit.

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
must not be exported into tracked `.atelier/` canonical records or treated as
Mission Control run projection data until an explicit run/session projection
contract exists.

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

Atelier should support repository-owned configurable issue workflows. Version 1
uses a fixed tracked `.atelier/workflow.yaml` file rather than a config-selected
policy path. The file defines shared statuses with explicit categories, named
issue workflows, terminal done states, built-in issue-type mappings, transition
rules, configured built-in validators with params, guidance templates, strict
configuration errors, and deferred features.

Example workflow:

```yaml
issue_types:
  bug: standard_review_proof
  closeout: standard_review_proof
  epic: standard_review_proof
  feature: standard_review_proof
  spike: lightweight_spike
  task: standard_review_proof
  validation: standard_review_proof

statuses:
  open:
    category: todo
  in_progress:
    category: active
  review:
    category: review
  validation:
    category: validation
  done:
    category: done

workflows:
  standard_review_proof:
    initial_status: open
    done_statuses: [done]
    transitions:
      start:
        from: [open]
        to: in_progress
      request_review:
        from: [in_progress]
        to: review
      request_validation:
        from: [in_progress, review]
        to: validation
      close:
        from: [validation]
        to: done
        required_fields: [close_reason]
        validators:
          - proof_attached
          - durable_current
```

Workflows should scale with risk. Small tasks should not require heavyweight
ceremony unless policy says so. The starter contract uses a standard
review/proof workflow for most issue types and a lighter reviewed spike
workflow that still records an inspectable close reason without requiring
first-class evidence.

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
- Split between start requirements and close requirements.
- Strict only where strictness protects correctness or coordination.

Rules should have severities:

- `info`
- `warn`
- `error`
- `policy`

Version 1 deliberately avoids workflow waivers. If a future product slice adds
waivers, they need an explicit contract and visibility model.

## Branches And Worktrees

Atelier should preserve Braid's agent worktree ergonomics.

Desired commands:

```text
atelier agent init <name>
atelier start atelier-z1p8
atelier worktree for atelier-z1p8
atelier finish atelier-z1p8
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

Version 1 built-in validators include:

- `durable_state_current`
- `review_complete`
- `evidence_attached`
- `validation_criteria_satisfied`
- `no_open_blockers`
- `no_blocking_lints`
- `git_worktree_clean`

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
atelier prime
atelier status
atelier issue list --ready
atelier issue show atelier-z1p8
atelier issue create
atelier issue transition atelier-z1p8 --options
atelier mission create
atelier mission status
atelier mission show atelier-k7mq
atelier mission add-work atelier-k7mq atelier-z1p8
atelier milestone create
atelier plan create
atelier evidence record --target issue/atelier-z1p8 --kind validation --result pass "summary"
atelier evidence record --target issue/atelier-z1p8 --kind test --result pass -- <command>
atelier lint
atelier export --check
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

- Define tracked canonical Markdown under `.atelier/`.
- Export deterministic per-record files.
- Add `export --check`.
- Add `rebuild`.
- Make SQLite rebuildable from committed Markdown records.
- Keep `.atelier/state.db` ignored as local runtime state; committed
  durable state lives under tracked `.atelier/` record directories.

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
- Resolved for the Markdown-first storage overhaul: `.atelier/` is the single
  project root. Canonical Markdown records and tracked project config live under
  `.atelier/`; `.atelier/state.db`, `.atelier/runtime/`, and `.atelier/cache/`
  contain ignored local projection/runtime/cache files such as SQLite state,
  identity, locks, diagnostics, and UI caches.
- Should sessions be exported, partially exported, or treated as local runtime
  metadata? Command diagnostics are resolved separately as local-only telemetry
  and do not answer this broader run/session policy.
- Which external artifact backend should be implemented after metadata-only
  path/URI evidence records?
- What shared or remote lock policy is needed after work association and
  Mission Control projections stabilize?
- What should the default workflow be for tiny tasks?
