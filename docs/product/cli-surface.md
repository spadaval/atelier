# CLI Surface Tiers

Atelier's public CLI presents the agent-native workflow first. Inherited
Chainlink utilities are not kept as command aliases once their replacement path
is documented; deleting old surfaces and their command code is preferred over
compatibility shims. This repository is a WIP product with few users, so
compatibility with obsolete command shapes, statuses, and output contracts is
usually harmful. Do not add staged deprecations, aliases, fallback readers, or
old-output shims unless a human explicitly asks for them for a specific
transition.

## Core

Core commands are stable enough to appear in `atelier --help` and are expected
in normal Agent Factory workflows:

- `atelier init`
- `atelier prime`
- `atelier status`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier search <query>`
- `atelier link add/remove/list`
- `atelier graph impact/tree`
- `atelier note add`
- `atelier mission create/show/list/status/update`
- `atelier mission add-work/add-blocker`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence record/show/list/attach`
- `atelier history`
- `atelier start`
- `atelier abandon`
- `atelier worktree for/status/merge/remove`
- `atelier maintenance delete`
- `atelier import-beads`
- `atelier lint`
- `atelier doctor`

`atelier init` is core tracker setup only. It creates `.atelier/` records,
`.atelier/config.toml`, local runtime storage, and root ignore rules; it does
not install editor or assistant hooks.

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

## Operator Jobs

The public command surface is organized around jobs an operator performs under
time pressure:

- Orient: answer what is active, ready, blocked, stale, or unsafe to change.
  Owned by `atelier status`, `atelier issue show/list`, `atelier mission show`,
  `atelier mission status`, `atelier history`, and `atelier search`.
- Select and run work: claim a clear slice, prepare the right worktree, start
  it, leave notes, advance or close it with proof, and abandon it locally when
  needed. Owned by
  `atelier issue ...`, `atelier worktree ...`, root `atelier start`, root
  `atelier abandon`, `atelier note add`, and `atelier evidence ...`.
- Coordinate mission progress: see linked work by state, blockers, evidence
  gaps, closeout readiness, and the next action for the mission. Owned by
  `atelier mission show`, `atelier mission status`, `atelier mission update`,
  and `atelier mission add-work/add-blocker`.
- Manage relationships: record dependencies, typed links, and impact when the
  next action depends on graph shape. Owned by `atelier dep ...`,
  `atelier link ...`, and `atelier graph impact/tree`.
- Check health: prove committed tracker state and local runtime are usable for
  handoff. Owned by `atelier lint`, `atelier doctor`, and closeout-required
  `atelier export --check`.

Normal workflow commands speak in product terms: issue, mission, worktree,
evidence, blocker, proof, closeout, and health. Advanced diagnostics may expose
workflow policy names, projections, cache repair, command telemetry, or raw
validator detail, but normal operators should only run them when a binding,
assignment, or closeout contract names them. Destructive maintenance is a third
category: commands such as `atelier maintenance delete issue <id> --force`
exist for explicit record surgery and must not appear as routine next actions.
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

There is no `atelier mission close` command in v1. Closing a mission uses
`atelier mission update <id> --status closed`, which routes through the
configured close transition validators. Reopening with
`atelier mission update <id> --status ready` does not run closeout validators.

Issue mutation commands are migrating toward Markdown-direct writes through
RecordStore followed by projection refresh. Projection-backed query commands
such as issue list, ready queues, search, graph impact/tree, lint, and Mission
Control views may use SQLite after freshness checks.
Issue creation and issue detail output print the canonical Markdown path under
`.atelier/issues/<id>.md` so large-field editing stays file-first. Human
footers point to editing that Markdown file, `atelier lint <id>`, and focused
drill-down commands rather than generic command dumps.
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
Markdown through RecordStore before refreshing the SQLite projection; rebuild
restores those records from Markdown. `atelier plan apply` validates authored
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
identity and state, tracker health, work counts, open blockers, missing proof,
closeout readiness, and one or two state-specific next actions. Verbose output
keeps the same sections but expands the record lists, evidence gaps, blocked
work, and degraded health details enough for a handoff transcript.

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
next domain command. `workflow check`, `issue transition --options`, `mission
status`, and `mission audit` are the supported inspection surfaces; removed
policy-debug commands do not replace them.
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

`atelier evidence add` and `atelier evidence capture` are predecessor shapes
that split one operator job into two verbs. New help and Agent Factory guidance
should teach `evidence record`; implementation may keep old entrypoints only as
internal migration scaffolding until the unified surface is shipped.
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
worktree plus current-export checks where they affect workflow transitions.
Root `atelier status`, `atelier mission status`, and `atelier issue transition
<id> --options` expose current-work orientation, so operators should not need
the hidden `atelier work start/status` command group for normal workflow.
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
should not appear as ordinary next actions from core workflow surfaces, and
their output should make clear that they are diagnostic tools rather than
required user workflow. `atelier diagnostics slow` summarizes local command
telemetry for command-performance investigations; it is visible help, but it is
not a normal mission or issue workflow step.

Installed-binary drift is distinct from malformed records. Normal tracker work
uses the installed `atelier` command. When a command reports that canonical
records use a schema the binary does not understand, operators should rebuild
and use `target/debug/atelier` for local CLI changes or update the installed
binary before continuing. Use `cargo run -- ...` only when a one-off rebuild
plus execution is specifically useful. When canonical Markdown is malformed,
the repair path remains `atelier lint`, editing the named record, and
`atelier export --check`; ordinary record syntax errors are not stale-binary
drift.

## Removed Behavior

The inherited command layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above.

Removed command surfaces:

- `mission view`; use `mission show`.
- The normal `work start/finish/status` group; use root `start`, root
  `abandon`, `issue close`, and `status`.
- Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`,
  `update`, `block`, `unblock`, `search`, `relate`, `related`, and `tree`; use
  `atelier issue ...`.
- Flat timer aliases such as `stop`, and the `timer` group.
- Legacy groups `archive`, `milestone`, `session`, `daemon`, `cpitd`, `usage`,
  `agent`, `locks`, and `sync`.
- Backup `import` and `export --format json|markdown`; use `import-beads` for
  predecessor imports and canonical `export`/`rebuild` for durable state.

## Low-Level Debug And Repair

The implementation may retain low-level commands such as `atelier rebuild` and
projection/export repair checks for development, diagnostics, migration, and
test evidence. They are not part of the normal product workflow. Public
orientation commands such as `atelier status`, `atelier doctor`, `atelier lint`,
and record-specific repair guidance should absorb routine cache recovery.

Internal helpers may remain only when a core workflow still uses them. For
example, session rows remain an implementation detail of current work
association, but there is no `session` command.

## Integration Or Experimental

Integration experiments must not define the product's default mental model.
Removed experiments should return as explicit core proposals rather than hidden
command groups.

- `atelier integrations claude install` installs the optional Claude Code
  integration. It writes `.claude/hooks`, `.claude/mcp`, `.claude/settings.json`,
  merges Atelier's MCP server into `.mcp.json`, and writes
  `.atelier/hook-config.json`. It reads `.atelier/config.toml` to confirm the
  current project layout and does not copy the bundled `resources/atelier/rules`
  tree into each repository.

Generic replacements should use domain language. `atelier graph impact <id>` is
the visible relation-impact command. `atelier link add/remove/list` owns typed
links, `atelier note add issue <id> "..."` owns issue activity notes, and
`atelier search <query>` owns text search. Impact follows hierarchy plus the
impact-bearing relation types `derived`, `caused-by`, and `falsifies`
transitively, and `assumption` one hop from the source. The inherited `cascade`
and `falsify` commands are removed so reassessment stays an explicit operator
action through graph, link, note, and lifecycle commands instead of an
assumption-specific command path.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

Canonical state is Markdown under tracked `.atelier/` records and checked with
lint and health gates. Local indexes are repaired transparently by normal
commands. `atelier export` remains a low-level repair renderer for committed
tracker state; backup JSON/Markdown formats are no longer command surfaces.
