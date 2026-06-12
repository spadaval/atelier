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
- `atelier evidence add/capture/show/list/attach`
- `atelier history`
- `atelier start`
- `atelier finish`
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
mission-control CLI surface for active mission health, evidence gaps, blockers,
validator freshness, closeout readiness, and next actions before any separate
projection or UI is required. `atelier mission audit <id>` is the closeout
contract surface: it maps authored mission validation expectations and linked
epic outcomes to current work, blockers, and attached evidence before a mission
can close.
`atelier evidence capture` runs a command and stores bounded stdout/stderr
summaries with command, exit status, result, timestamp, and optional target
metadata so validation proof does not require manual transcript copy/paste.
`atelier history` is the canonical project-history view. Repo-wide history and
scoped forms such as `atelier history --mission <id>`, `atelier history --issue
<id>`, and `atelier history --epic <id>` read canonical activity sidecars,
first-class records, evidence, and record links. History defaults to newest
first with bounded output, supports event kind, actor, time, and scope filters,
and separates canonical tracker history from local runtime diagnostics. Issue
and mission show surfaces may include compact recent activity or record context,
but they point to scoped history for full activity instead of expanding
unbounded logs.
Root `atelier start <issue-id>` and `atelier finish [issue-id]` are the normal
work lifecycle commands. They store local work association in runtime state and
enforce clean worktree plus current-export checks where they affect workflow
transitions. Root `atelier status`, `atelier mission status`, and `atelier issue
transition <id> --options` expose current-work orientation, so operators should
not need the hidden `atelier work start/finish/status` command group for normal
workflow. Worktree helpers expose scan-friendly JSON status, create/remove
associated Git worktrees, prepare local runtime state in new worktrees, and run
`worktree_setup` hooks from the configured workflow policy.
Mission closeout is ready only when all linked work is closed, required evidence
is attached, workflow validators pass, and the Git worktree is clean.

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

## Removed Behavior

The inherited command layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above.

Removed command surfaces:

- `mission view`; use `mission show`.
- The normal `work start/finish/status` group; use root `start`, root `finish`,
  and `status`.
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
