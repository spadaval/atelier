# CLI Surface Tiers

Atelier's public CLI presents the agent-native workflow first. Inherited
Chainlink utilities are not kept as command aliases once their replacement path
is documented; deleting old surfaces and their command code is preferred over
compatibility shims. Compatibility aliases are retained only when a staged
migration protects current Agent Factory guidance or existing operator habits.

## Core

Core commands are stable enough to appear in `atelier --help` and are expected
in normal Agent Factory workflows:

- `atelier init`
- `atelier status`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier mission create/show/list/status/update`
- `atelier mission add-work/add-blocker`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence add/show/list/attach`
- `atelier workflow validate`
- `atelier work start/finish/status`
- `atelier worktree for/status/merge/remove`
- `atelier export` and `atelier export --check`
- `atelier rebuild`
- `atelier import-beads`
- `atelier lint`
- `atelier doctor`

`atelier init` is core tracker setup only. It creates `.atelier/` records,
`.atelier/config.toml`, local runtime storage, and root ignore rules; it does
not install editor or assistant hooks.

`atelier status` is the root checkout signpost. It summarizes active work,
active mission focus, ready work count, tracker freshness, and the next
mission/work/health drill-down commands. It does not replace `mission status`;
it points operators to the scoped status surface that owns closeout readiness.

Mission lifecycle statuses are `draft`, `ready`, `active`, and `closed`.
Mission creation defaults to `ready`; `atelier mission start <id>` transitions
the selected mission to `active` and transitions any previous active mission
back to `ready` when `--switch` is supplied. Legacy `open` mission records are
read as `ready`, and legacy `data.active` records are read as `active` until
the canonical record is next rewritten. `atelier mission list --status open`
remains a compatibility filter for all non-closed missions, and
`atelier mission update <id> --status open` is accepted as a staged alias for
`ready`.

There is no `atelier mission close` command in v1. Closing a mission uses
`atelier mission update <id> --status closed`, which routes through the
configured close transition validators. Reopening with
`atelier mission update <id> --status ready` does not run closeout validators.

Issue mutation commands are migrating toward Markdown-direct writes through
RecordStore followed by projection refresh. Projection-backed query commands
such as list, ready, search, impact, lint, and Mission Control views may use
SQLite after freshness checks.
Issue creation and issue detail output print the canonical Markdown path under
`.atelier/issues/<id>.md` so large-field editing stays file-first. Human
footers point to editing that Markdown file, `atelier lint <id>`, and focused
drill-down commands rather than generic command dumps.

First-class mission, milestone, plan, evidence, relationship, workflow validation,
and work lifecycle commands are now core as a staged implementation. Mission,
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
projection or UI is required.
Work lifecycle commands store local work association in runtime state and
enforce clean worktree plus current-export checks where they affect workflow
transitions. Worktree helpers expose scan-friendly JSON status, create/remove
associated Git worktrees, rebuild SQLite in new worktrees, and run
`worktree_setup` hooks from the configured workflow policy.
Mission closeout is ready only when all linked work is closed, required evidence
is attached, workflow validators pass, and the Git worktree is clean.

## Removed Compatibility

The legacy compatibility layer has been classified and removed from the public
command surface. The default classification for an inherited or duplicate
surface is `delete` unless it is in the core list above or required by
`AGENTFACTORY.md`.

Removed command surfaces:

- `mission view`; use `mission show`.
- Flat issue aliases such as `create`, `show`, `list`, `ready`, `close`,
  `update`, `block`, `unblock`, `search`, `relate`, `related`, and `tree`; use
  `atelier issue ...`.
- Flat timer aliases `start` and `stop`, and the `timer` group.
- Legacy groups `archive`, `milestone`, `session`, `daemon`, `cpitd`, `usage`,
  `agent`, `locks`, and `sync`.
- Backup `import` and `export --format json|markdown`; use `import-beads` for
  predecessor imports and canonical `export`/`rebuild` for durable state.

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

Generic replacements should use domain language. `atelier issue impact <id>` is
the visible relation-impact command. Impact follows hierarchy plus the
impact-bearing relation types `derived`, `caused-by`, and `falsifies`
transitively, and `assumption` one hop from the source. The inherited `cascade`
and `falsify` commands are removed so reassessment stays an explicit operator
decision through `issue impact`, `issue label`, `issue comment`, or `issue close`
instead of an assumption-specific command path.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

Canonical state is Markdown under tracked `.atelier/` records, checked with
lint/export compatibility gates and indexed into SQLite with `atelier rebuild`.
`atelier export` remains a compatibility and repair renderer during migration;
backup JSON/Markdown formats are no longer command surfaces.
