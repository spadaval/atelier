# CLI Surface Tiers

Atelier's public CLI presents the agent-native workflow first. Inherited
Chainlink utilities are not kept as command aliases once their replacement path
is documented; deleting old surfaces and their command code is preferred over
compatibility shims. This repository is a WIP product with few users, so
compatibility with obsolete command shapes, statuses, and output contracts is
usually harmful. Do not add staged deprecations, aliases, fallback readers, or
old-output shims unless a human explicitly asks for them for a specific
transition.

Per [ADR 0006](../adr/0006-agent-guidance-ownership-boundary.md), this file
and Atelier help own the repository's tactical operator guidance. Agent Factory
coordinates agents, but it should route repo-specific command choice,
workflow-recovery, readiness, and closeout detail to Atelier-owned command and
doc surfaces instead of duplicating that contract.

## Workflow-First Core

Workflow-first commands are stable enough to appear in `atelier --help` and are
the normal repo-owned operational path that Agent Factory should reference:

- `atelier init`
- `atelier prime`
- `atelier status`
- `atelier issue ...`
- `atelier search <query>`
- `atelier graph impact/tree`
- `atelier mission create/show/list/status/update/note`
- `atelier mission add-work/unlink/add-blocker`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence record/show/attach/list`
- `atelier history`
- `atelier issue note <id> "..."`
- `atelier start`
- `atelier abandon`
- `atelier worktree for/status/merge/remove`
- `atelier integrations ...`
- `atelier maintenance ...`
- `atelier import-beads <path>`
- `atelier lint`
- `atelier doctor`

`atelier init` is core tracker setup only. It creates `.atelier/` records,
`.atelier/config.toml`, `.atelier/workflow.yaml`, local runtime storage, and
root ignore rules. It does not install editor or assistant hooks. When
`--import-beads` is supplied, it imports the standard repo-local Beads backup
from `.beads/issues.manual.jsonl`; otherwise it may mention that migration
input was detected but must not silently convert it. Its default next steps
must not route a fresh checkout directly to issue creation before `atelier
lint` confirms the committed tracker and workflow configuration are valid.

`atelier prime` is the recovery and onboarding signpost. It explains how an
agent should operate in this repository, where durable tracker state lives, what
local runtime/cache state is ignored, and which command families matter. It is
mostly static guidance with a small dynamic header for repository path, active
mission, active work, ready work, and tracker freshness. It is not a live status
dashboard, full command reference, or implementation tutorial; every command it
prints must have a concrete reason.

`atelier status` is the root checkout signpost. It summarizes active work,
active mission focus, ready work count, tracker freshness, and the next
mission/work/health drill-down commands. It does not replace `mission status`;
it points operators to the scoped status surface that owns closeout readiness.

`atelier --help` should expose the normal operator path first: orient, inspect
missions and issues, start or abandon active work, manage blockers, record
proof, inspect history, and run health checks. Low-level state diagnostics such
as export/rebuild, predecessor imports, raw workflow diagnostics, and command
telemetry are not normal root-help entries; `doctor`, targeted error messages,
and migration docs may name them when they are relevant. Destructive
maintenance may remain visible as an explicit danger-zone command.

## Visible Help Contract

Every help-visible command group needs one job, one default information budget,
one quiet-output contract, and a named drill-down path. Quiet mode stays terse:
IDs, counts, paths, status tokens, and pass/fail tokens only.

### Workflow-First Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `init` | Create tracker scaffolding in a repo that does not have Atelier yet. | Created or reused paths plus workflow setup, optional Beads migration detection, and verification commands before issue creation. | Created path(s) and a success token. | `lint`, `prime`, `status`, inspect `.atelier/config.toml` and `.atelier/workflow.yaml`. |
| `prime` | Recovery and onboarding signpost for this checkout. | Small dynamic repo header plus concise workflow guidance and named commands. | Repo path, active mission/work IDs, ready count, freshness token. | `status`, `mission status`, `issue list --ready`. |
| `status` | Root orientation for the current checkout. | Active work, active mission, ready count, tracker freshness, and the next work/mission/health commands. | IDs, counts, and freshness token only. | `mission status`, `issue show <id>`, `issue list --ready`, `doctor`. |
| `start` | Establish active local work on one issue. | Confirmation, local association state, and the next work commands. | Issue ID and success token. | `issue show <id>`, `worktree for <id>`, `status`. |
| `abandon` | Drop the local work association without mutating tracker status. | Confirmation, recorded reason, and any remaining local cleanup hint. | Issue ID and cleared token. | `status`, `worktree status`, `issue show <id>`. |
| `issue` | Create, list, show, update, transition, close, note, and manage issue-owned blockers. | Queue or detail views using the shared human-output grammar; detail reads name the canonical Markdown path and next commands. Blocker mutations name the blocked issue and blocker issue, blocker inspection stays under `issue blocked`, and note entry appends activity without field mutation. | IDs, status tokens, changed fields, blocker IDs, and canonical paths. | `issue show <id>`, `issue note <id> "..."`, `issue transition <id> --options`, `issue list --blocked`, `issue blocked [<id>]`, edit the Markdown record, `history --issue <id>`. |
| `search` | Search record text when the operator does not know the exact ID yet. | Bounded queue grouped by readiness or priority when useful, with the search query echoed. | Matching IDs only. | `issue show <id>`, `history`, `graph tree --compact`. |
| `graph` | Inspect cross-record hierarchy and downstream impact shape. | `impact` prints a bounded downstream set across mission and issue relationships; `tree` prints compact mission/issue hierarchy cues unless a broader tree was explicitly requested. | IDs, counts, kinds, and status or priority tokens only. | `mission show <id>`, `issue show <id>`, `issue list --blocked`. |
| `mission` | Create, focus, inspect, update, note, and coordinate durable missions. | `show` is the rich mission detail view; `status` is the compact health and next-action view; `list` stays queue-oriented; lifecycle changes stay on `update`; note entry appends mission activity without field mutation. | IDs, counts, lifecycle tokens, and closeout-readiness token. | `mission show <id>`, `mission note <id> "..."`, `mission status [<id>]`, `mission audit <id>`, `history --mission <id>`. |
| `plan` | Author, inspect, revise, link, and apply durable plans. | `show` and `list` are readable plan views; `apply` prints preview or created-record summaries rather than raw JSON internals. | Plan IDs, affected-record counts, and status tokens. | `plan show <id>`, `mission show <id>`, `history`. |
| `evidence` | Record and inspect proof records. | `record` is the default proof-capture workflow; `show` and `list` inspect existing evidence; output names target, kind, result, and reusable IDs. | Evidence IDs, target IDs, result tokens, and stored command status only. | `evidence show <id>`, `history --issue <id>`, `issue show <id>`. |
| `history` | Inspect canonical repo, mission, issue, or epic activity. | Newest-first bounded activity feed with scope and filter context echoed. | Event counts, scoped IDs, and timestamps only. | Broaden or narrow with `--mission`, `--issue`, `--epic`, `--event-kind`, `--actor`, or `--since`; return to `issue show` or `mission show` for current state. |
| `worktree` | Create, inspect, merge, repair, and remove issue worktrees. | `for`, `merge`, `repair`, and `remove` acknowledge the affected issue/path; `status` stays scan-friendly and bounded. | Issue IDs, paths, and worktree-state tokens. | `worktree status`, `status`, `issue show <id>`. |

### Specialized But Visible Surfaces

| Surface | Job | Default output | Quiet output | Drill-down path |
| --- | --- | --- | --- | --- |
| `maintenance` | Explicit destructive record surgery only. | Clear target and consequence summary before deletion, then confirmation of the deleted record. | Deleted ID and kind only. | `history`, `lint`, and Git inspection when recovery is needed. |
| `integrations` | Install or refresh optional external-tool integrations. | Acknowledges the targeted integration and the managed files it wrote or verified. | Integration name and success token. | Re-run the same integration command or inspect the managed files. |
| `import-beads` | Import a repo-local or explicit Beads backup into canonical Atelier records. | Import summary, created-record counts, and any follow-up lint or review guidance. | Imported counts and success token. | `history`, `issue show <id>`, `lint`. |
| `diagnostics` | Inspect local command telemetry for Atelier itself. | Stable diagnostic output for the named probe, currently `slow`; hidden or advanced only, never a normal mission or issue next action and never the source of ready-work, blocker, validation, or closeout decisions. | Same diagnostic result trimmed to essential rows or counts. | `doctor`, performance follow-up issues, or the owning architecture docs. |
| `lint` | Validate canonical tracker records and committed workflow configuration. | Pass summary or named record, workflow config, and file errors with repair guidance. | Pass/fail token and offending IDs or paths only. | Edit the named record or workflow config, rerun `lint`, `doctor`. |
| `doctor` | Validate runtime, install, and derived-state health; repair ignored local state when `--fix` is supplied. | Named health checks, degraded-state reason, and repair guidance. With `--fix`, reports each ignored runtime/cache/projection repair and refuses to edit tracked `.atelier/` canonical records. | Pass/fail token and degraded check names only. | `lint`, edit named canonical records, `status`. |

## Operator Jobs

The public command surface is organized around jobs an operator performs under
time pressure:

- Orient: answer what is active, ready, blocked, stale, or unsafe to change.
  Owned by `atelier status`, `atelier issue show/list`, `atelier mission show`,
  `atelier mission status`, `atelier history`, and `atelier search`.
- Select and run work: choose a clear slice, prepare the right worktree, start
  it, leave notes, advance or close it with proof, and abandon it locally when
  needed. Owned by
  `atelier issue ...`, `atelier worktree ...`, root `atelier start`, root
  `atelier abandon`, record-specific note commands, and `atelier evidence ...`.
- Coordinate mission progress: see linked work by state, blockers, evidence
  gaps, closeout readiness, and the next action for the mission. Owned by
  `atelier mission show`, `atelier mission status`, `atelier mission update`,
  `atelier mission add-work/unlink/add-blocker`, and `atelier mission audit`.
- Manage relationships: record issue blockers and inspect cross-record impact
  when the next action depends on graph shape. Owned by issue blocker
  subcommands, mission work-link subcommands, evidence attachment, plan
  relationship commands, and `atelier graph impact/tree`.
- Check health: prove committed tracker state and local runtime are usable for
  handoff. Owned by `atelier lint` and `atelier doctor`; low-level state
  diagnostics such as export/rebuild are not normal handoff commands.

Normal workflow commands speak in product terms: issue, mission, worktree,
evidence, blocker, proof, closeout, and health. Advanced diagnostics may expose
workflow policy names, projections, cache repair, command telemetry, JSON
summaries, or raw validator detail, but normal operators should only run them
when a binding, assignment, or closeout contract names them. JSON emitted by
diagnostics commands is an Atelier-maintenance interface for local telemetry
and performance analysis, not an automation contract for selecting work,
proving validation, deciding blockers, or closing missions. Destructive
maintenance is a third category: commands such as `atelier maintenance delete
issue <id> --force` exist for explicit record surgery and must not appear as
routine next actions.
Common read and orientation commands correct record-kind mistakes directly:
when an operator passes an existing mission, plan, or evidence ID to an issue
command, the error names the actual record kind and suggests the matching show
command. Genuinely unknown IDs remain concise not-found errors without implying
a false record match.

Every command-consolidation proposal must pass a red-tape check before it is
implemented: the new shape must remove a real duplicate, reduce the chance of
choosing the wrong command, and shorten the path from question to observable
answer. A consolidation that merely moves ritual into a new umbrella command,
adds mandatory ceremony to ordinary work, or hides the domain next action behind
diagnostic jargon fails this check.

Mission lifecycle statuses are `draft`, `ready`, `active`, and `closed`.
Mission creation defaults to `ready`; `atelier mission start <id>` transitions
the selected mission to `active` and transitions any previous active mission
back to `ready` when `--switch` is supplied. Mission commands do not accept
`open` as a mission-status alias and do not read legacy `data.active` state;
committed mission records should be migrated directly to the lifecycle status
they mean.

Mission closeout currently uses `atelier mission update <id> --status closed`,
which runs the mission closeout gates before it commits the lifecycle change.
Reopening with `atelier mission update <id> --status ready` does not run
closeout validators.

Issue mutation commands are migrating toward Markdown-direct writes through
RecordStore followed by projection refresh. Projection-backed query commands
such as issue list, ready queues, search, graph impact/tree, lint, and Mission
Control views may use SQLite after freshness checks.
Issue creation and issue detail output print the canonical Markdown path under
`.atelier/issues/<id>.md` so large-field editing stays file-first. Human
footers point to editing that Markdown file, `atelier lint <id>`, and focused
drill-down commands rather than generic command dumps.
`atelier issue list --status <status>` filters by exact configured workflow
status only, with `all` as the only special token. Derived status categories
are a separate concept and use `atelier issue list --category <category>`.
Category values are the exact category names from `.atelier/workflow.yaml`
such as `todo`, `active`, `blocked`, `review`, `validation`, and `done`; status
tokens such as `in_progress` are not category aliases.
`atelier issue create` has one work-type decision. Use `--issue-type` for the
canonical type (`bug`, `closeout`, `epic`, `feature`, `spike`, `task`, or
`validation`) or use a template preset whose default type is documented by the
template name (`bug` -> `bug`, `feature` -> `feature`, research/investigation ->
`spike`, audit -> `validation`, refactor/continuation -> `task`). Templates
also provide default labels, priorities, and body scaffolding, and those
defaults still apply when `--parent` or a matching explicit `--issue-type` is
supplied. Conflicting type choices fail with guidance rather than silently
creating a surprising record.

First-class mission, milestone, plan, evidence, relationship, and work lifecycle
commands are now core as a staged implementation. Mission,
milestone, plan, evidence, and issue lifecycle mutations write canonical
Markdown through RecordStore before refreshing the SQLite projection; local
projection repair is normally transparent or routed through `doctor --fix`.
`atelier plan apply` validates authored
bulk-plan JSON, supports dry-run and validate-only previews, creates issue and
record graphs in canonical Markdown, normalizes issue dependency fields, writes
canonical relationship buckets, and refreshes the projection after successful
canonical writes. `atelier mission show` is the rich mission detail read:
it summarizes linked plans, milestones, evidence, and work grouped by ready,
blocked, done, and backlog state. `atelier mission status [<id>]` is the
mission-control CLI surface for active mission health, mission proof gaps,
blockers, record health, docs/help drift, ignored-test review, dirty worktree
state, closeout readiness, and next actions before any separate projection or UI
is required.

`atelier mission status` without an ID defaults to the active mission when one
exists; otherwise it reports the available ready missions and the command to
select one. With an ID, the command is scoped to that mission regardless of the
active runtime association. Default output is compact and answers: mission
identity and state, tracker health, work counts, selectable work, blocked work,
open blockers, missing proof, closeout readiness, and one or two state-specific
next actions. Selectable-work rows name the issue to start, its parent context,
and whether proof is already attached; blocked-work rows name the blocked issue,
the blocker IDs, parent context, and proof state. Verbose output keeps the same
sections but expands the record lists, evidence gaps, blocked work, and degraded
health details enough for a handoff transcript.

State-specific next actions are part of the command contract:

- `draft`: show missing readiness fields and point to record editing,
  `atelier lint <mission-id>`, or `atelier mission update <id> --status ready`.
- `ready`: show ready work and the command to start or switch mission focus.
- `active`: show active work, ready work, blockers, evidence gaps, and the next
  issue, evidence, or health command that advances the mission.
- `blocked`: show the open blocker records first and point to the specific
  blocker or dependent issue to resolve.
- `close-ready`: show the closeout command only after linked work is closed,
  required proof is attached to accountable work, the contract audit passes,
  health gates are current, and the worktree is clean.
- `closed`: show the close reason, closeout evidence or closeout issue, and
  history/audit drill-down commands without suggesting new implementation work.

`atelier mission audit <id>` is not a normal daily status command. Its fate is
to remain a closeout drill-down and, where practical, become the verbose
closeout section behind `atelier mission status --closeout` or equivalent. The
audit maps authored mission validation expectations and linked epic outcomes to
closed work, blockers, and evidence on accountable implementation, review,
validation, and closeout issues. Raw workflow validator names are diagnostic
detail; normal closeout output names the operator-facing blocker class and the
next domain command. `atelier lint` owns committed workflow/config validity,
`issue transition --options` owns issue-level readiness inspection, and
`mission status` plus `mission audit` own mission closeout inspection; removed
policy-debug commands do not replace them. Fast docs/help drift guards for
`AGENTS.md`, `AGENTFACTORY.md`, product command docs, visible root help, and
obsolete command-test references belong in `atelier lint` or an explicitly
named development-only diagnostic, not a required normal workflow command.
`atelier evidence record` is the normal evidence-recording surface. It records
manual summaries, command transcripts, audits, failed validations, deferred
proof, and artifact references as one operator workflow. The target is supplied
as one low-friction argument:

```text
atelier evidence record --target issue/<id> --kind validation --result pass "summary"
atelier evidence record --target issue/<id> --kind test --result pass -- <command>
```

The target syntax is `<kind>/<id>`. Version 1 accepts `issue/<id>` as the normal
accountable target. Direct `mission/<id>` targets are reserved for legacy
imports, migration notes, or explicit closeout mirroring; normal mission and
epic readiness reads proof from linked accountable child issues. The command
mode preserves the old capture behavior by storing the command, exit status,
success flag, timestamp, result, and bounded stdout/stderr summaries so
validation proof does not require manual transcript copy/paste.
Evidence target links use the relation role `validates`; evidence classifications
such as `validation`, `test`, or `review` belong in `--kind`.

`atelier evidence attach` is not the default proof-capture workflow. Its only
distinct job is reusing an already-created evidence record on an additional
target, such as an explicit closeout mirror or another accountable issue. New
help, next-action text, and Agent Factory guidance should teach
`atelier evidence record`; they should mention `evidence attach` only when the
operator is reusing existing proof instead of capturing it.
The predecessor evidence add/capture shapes split one operator job into two
verbs. New help and Agent Factory guidance should teach `evidence record`;
implementation may keep old entrypoints only as internal migration scaffolding
until the unified surface is shipped.
`atelier history` is the canonical project-history view. Repo-wide history and
scoped forms such as `atelier history --mission <id>`, `atelier history --issue
<id>`, and `atelier history --epic <id>` read canonical activity sidecars,
first-class records, evidence, and record links. History defaults to newest
first with bounded output, supports event kind, actor, time, and scope filters,
and separates canonical tracker history from local runtime diagnostics. Issue
and mission show surfaces may include compact recent activity or record context,
but they point to scoped history for full activity instead of expanding
unbounded logs.
Root `atelier start <issue-id>`, `atelier issue close <issue-id> --reason "..."`,
and `atelier abandon [issue-id] --reason "..."` are the normal work lifecycle
commands. They store local work association in runtime state and enforce clean
worktree plus current derived-state health checks where they affect workflow
transitions. A separate durable claim system is not part of the normal workflow
unless a later assignment policy justifies it.
Root `atelier status`, `atelier mission status`, and `atelier issue transition
<id> --options` expose current-work orientation, so operators should not need
the removed work-status helper or any legacy work-start path for
normal workflow.
Worktree helpers expose scan-friendly JSON status, create/remove associated Git
worktrees, and prepare local runtime state in new worktrees.
Workflow-defined hooks are deferred in v1 and are not part of the normal
worktree helper contract.
Mission closeout is ready only when all linked work is closed, required proof is
attached to the accountable implementation, review, validation, or closeout
work, the contract audit passes, linked issue records are parseable, docs/help
and ignored-test review gates are current, and the Git worktree is clean.

## Cache Transparency

The local SQLite projection and cache are implementation details. Normal
operators should not need to know that they exist, refresh them manually, or
interpret projection freshness as a product concept. Core read and mutation
commands must transparently keep local projection state usable, and degraded
states must be reported as record or workflow repair problems rather than cache
maintenance chores.

Low-level debug and repair commands may expose cache mechanics when diagnosing
Atelier itself. `atelier export`, `atelier rebuild`, and similar repair checks
must not appear as ordinary next actions from core workflow surfaces or root
help, and their output should make clear that they are diagnostic tools rather
than required user workflow. `doctor --fix` is the normal explicit repair path
for ignored local runtime/cache/projection state and must not edit tracked
`.atelier/` canonical records. `atelier diagnostics slow` summarizes local
command telemetry for command-performance investigations; it is an advanced
diagnostic, not a normal mission or issue workflow step. Its JSON is stable for
diagnostic tooling and local analysis, but normal recipes must not parse it to
decide ready work, validation status, blockers, evidence coverage, or closeout
readiness.

Installed-binary drift is distinct from malformed records. Normal tracker work
uses the installed `atelier` command. When a command reports that canonical
records use a schema the binary does not understand, operators should rebuild
and use `target/debug/atelier` for local CLI changes or update the installed
binary before continuing. Use `cargo run -- ...` only when a one-off rebuild
plus execution is specifically useful. When canonical Markdown is malformed,
the repair path remains `atelier lint`, editing the named record, and
rerunning `atelier lint`/`atelier doctor`; ordinary record syntax errors are
not stale-binary drift.
Stale projection and invalid canonical-record diagnostics should present one
ordered recovery path, preserve the original blocked command, and keep the
specific record path or parser detail visible so operators do not cycle through
export, rebuild, lint, and read commands blindly.

## Removed Behavior

The inherited command layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above.

## Compatibility Classification

| Shape | Disposition | Reason | Replacement or boundary |
| --- | --- | --- | --- |
| Evidence add predecessor | Remove | Splits manual proof capture away from the unified evidence workflow. | `atelier evidence record --target ... "summary"` |
| Evidence capture predecessor | Remove | Splits transcript capture away from the same proof workflow. | `atelier evidence record --target ... -- <command>` |
| `atelier evidence attach` | Keep with distinct purpose | Needed only when an existing evidence record is being mirrored or reused on another accountable target. | Do not teach as the normal first proof step. |
| `atelier export --check` | Low-level diagnostic | Cache/projection state should be transparent; normal health and closeout routes use `lint`, `doctor`, mission status/audit, and issue transition readiness. | Hidden/advanced diagnostic only; do not teach as a normal handoff command. |
| `atelier export` | Low-level diagnostic | Deterministic repair/render mechanics are implementation details. | Hidden/advanced diagnostic only; `doctor --fix` owns normal explicit local repair. |
| `atelier rebuild` | Low-level diagnostic | Projection rebuild is cache repair, not a product workflow. | Hidden/advanced diagnostic only; `doctor --fix` owns normal explicit local repair. |
| Hidden `issue quick/subissue/search/relate/tree/tested` helpers | Remove | Replacement commands are clear enough; hidden callable aliases are rediscovery risk. | Public workflows use `issue create/list/show/update/transition/close/block/unblock/blocked`, root `search`, cross-record `graph`, record-specific notes, `evidence`, and `status`. |
| Hidden work-status helper and any legacy work-start path | Remove | Duplicate lifecycle paths obscure the workflow-backed root commands, and work-start under that removed group is no longer supported. | Docs and help teach root `start`, root `abandon`, `issue close`, `status`, and `worktree`. |
| `mission view` | Remove | Duplicate of the richer mission detail surface. | `mission show` |
| Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`, `update`, `block`, `unblock`, `relate`, `related`, and `tree` | Remove | Duplicate verbs make the command surface harder to learn and easier to misroute. | `issue` owns issue lifecycle and blockers; `graph` owns cross-record impact/tree inspection. |
| Generic `atelier link` | Remove | Relationship ownership belongs to record-specific commands, and the generic surface misrepresents mission support. | Mission work links use `mission add-work/unlink`; issue blockers use issue commands; evidence uses evidence commands; graph inspects impact. |
| Backup `import` plus `export --format json|markdown` | Remove | Backup-oriented predecessor formats are not the target durable contract. | `init --import-beads` and temporarily hidden/manual `import-beads` for migration. |
| `cascade` and `falsify` | Remove | Relationship-specific verbs hide the broader graph model and encourage one-off command paths. | `graph impact`, record-specific note commands, and lifecycle commands. |

Removed command surfaces:

- `mission view`; use `mission show`.
- The normal `work start` and `work status` group; use root `start`, root
  `abandon`, `issue close`, and `status`.
- Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`,
  `update`, `block`, `unblock`, `search`, `relate`, `related`, and `tree`; use
  `atelier issue ...`.
- Flat timer aliases such as `stop`, and the `timer` group.
- Legacy groups `archive`, `milestone`, `session`, `daemon`, `cpitd`, `usage`,
  `agent`, `locks`, and `sync`.
- Backup `import` and `export --format json|markdown`; use `import-beads` for
  predecessor imports until `init --import-beads` owns the normal migration
  path.

When a removed or commonly retried command is rejected, the CLI keeps the
command unsupported and appends a corrective replacement. Examples include
`finish`, `current-work`, `issue new`, `work start`, `archive`, `session`, and
`timer`. `workflow check` remains a hidden low-level diagnostic, but unsupported
forms such as JSON mode also point operators to `issue transition <id>
--options`, `mission status`, `lint`, and `doctor` instead of teaching it as the
normal workflow path.

## Low-Level Debug And Repair

The implementation may retain low-level commands such as `atelier export`,
`atelier rebuild`, raw workflow diagnostics, `import-beads`, and command
diagnostics for development, diagnostics, migration, and test evidence. They
are not part of the normal product workflow or root help. Public orientation
commands such as `atelier status`, `atelier doctor`, `atelier lint`, and
record-specific repair guidance should absorb routine cache recovery.

Internal helpers may remain only when a core workflow still uses them. For
example, session rows remain an implementation detail of current work
association, but there is no `session` command.

## Integration Or Experimental

Integration experiments must not define the product's default mental model.
The Claude Code integration surface is removed rather than supported as an
Atelier product feature. Future integrations should return as explicit product
proposals with their own operator value, not as hidden command groups.

Generic replacements should use domain language. `atelier graph impact <id>` is
the visible cross-record impact command, and `atelier search <query>` owns text
search. Impact follows mission work links, issue hierarchy, blocking
relationships, and any explicitly impact-bearing relationship types that remain
after generic `atelier link` is removed. The inherited `cascade` and `falsify`
commands are removed so reassessment stays an explicit operator action through
graph, record-specific notes, and lifecycle commands instead of an
assumption-specific command path.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

Canonical state is Markdown under tracked `.atelier/` records and checked with
lint and health gates. Local indexes are repaired transparently by normal
commands or explicitly by `doctor --fix`. `atelier export` and `atelier
rebuild` remain low-level diagnostics only; backup JSON/Markdown formats are no
longer command surfaces.
