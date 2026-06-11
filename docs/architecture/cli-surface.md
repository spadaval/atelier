# CLI Surface Tiers

Atelier's public CLI presents the agent-native workflow first. Inherited
Chainlink utilities are not kept as command aliases once their replacement path
is documented; deleting old surfaces and their command code is preferred over
compatibility shims.

## Core

Core commands are stable enough to appear in `atelier --help` and are expected
in normal Agent Factory workflows:

- `atelier init`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier mission create/show/list/update`
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

Issue mutation commands are migrating toward Markdown-direct writes through
RecordStore followed by projection refresh. Projection-backed query commands
such as list, ready, search, impact, lint, and Mission Control views may use
SQLite after freshness checks.

First-class mission, milestone, plan, evidence, relationship, workflow validation,
and work lifecycle commands are now core as a staged implementation. Mission,
milestone, plan, and evidence mutations persist through SQLite and immediately
refresh deterministic Markdown projection under `.atelier-state/`; rebuild
restores those records from Markdown. `atelier plan apply` validates authored
bulk-plan JSON, supports dry-run and validate-only previews, creates issue and
record graphs atomically, normalizes issue dependency fields, writes canonical
relationship buckets, and refreshes or checks canonical export according to the input's
`apply.export` option. `atelier mission show` is the single rich mission read:
it summarizes linked plans, milestones, evidence, and work grouped by ready,
blocked, done, and backlog state.
Work lifecycle commands store local work association in runtime state and
enforce clean worktree plus current-export checks where they affect workflow
transitions. Worktree helpers expose scan-friendly JSON status, create/remove
associated Git worktrees, rebuild SQLite in new worktrees, and run
`worktree_setup` hooks from `atelier.workflow.yaml`.

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

Canonical state is Markdown under `.atelier-state/`, checked with
`atelier export --check` and indexed into SQLite with `atelier rebuild`.
`atelier export` is the deterministic canonical renderer; backup JSON/Markdown
formats are no longer command surfaces.
