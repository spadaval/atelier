# CLI Surface Tiers

Atelier's public CLI should present the agent-native workflow first. Inherited
Chainlink utilities may remain callable while migration work is in progress, but
they should not appear as peer commands in primary help unless they are
explicitly promoted by a follow-up issue or ADR.

## Core

Core commands are stable enough to appear in `atelier --help` and are expected
in normal Agent Factory workflows:

- `atelier init`
- `atelier issue ...`
- `atelier dep add/remove/list`
- `atelier mission create/show/list/update/view`
- `atelier plan create/show/list/revise/link/apply`
- `atelier evidence add/show/list`
- `atelier link add/remove/list`
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

First-class mission, milestone, plan, evidence, typed-link, workflow validation,
and work lifecycle commands are now core as a staged implementation. Mission,
milestone, plan, and evidence mutations persist through SQLite and immediately
refresh deterministic Markdown projection under `.atelier-state/`; rebuild
restores those records from Markdown. `atelier plan apply` validates authored
bulk-plan JSON, supports dry-run and validate-only previews, creates issue and
record graphs atomically, normalizes issue dependency fields, writes typed
links, and refreshes or checks canonical export according to the input's
`apply.export` option. `atelier mission view` summarizes linked plans,
milestones, evidence, and work grouped by ready, blocked, done, and backlog
state. Work lifecycle commands store local work association in runtime state and
enforce clean worktree plus current-export checks where they affect workflow
transitions. Worktree helpers expose scan-friendly JSON status, create/remove
associated Git worktrees, rebuild SQLite in new worktrees, and run
`worktree_setup` hooks from `atelier.workflow.yaml`.

## Hidden Compatibility

Compatibility commands remain callable for existing scripts, imported habits, or
migration paths, but docs should prefer their core replacements and primary help
should hide them:

- Flat issue aliases: `create`, `show`, `list`, `ready`, `close`, `update`,
  `block`, `unblock`, `search`, `relate`, `related`, `tree`, and similar aliases
  for `atelier issue ...`.
- `timer` and flat `start`/`stop`; normal tracked work should use
  `atelier work start/finish/status`.
- `archive`, because archive state is inherited issue lifecycle behavior rather
  than a target workflow primitive.
- Legacy `milestone`, because first-class checkpoint records are projected as
  `records(kind = milestone)` and created through bulk-plan apply until a
  dedicated visible checkpoint command is promoted.
- `session`, because durable run/session accounting is deferred. Session state
  may still be updated by `atelier work start` as local runtime context.
- `agent`, `locks`, and `sync`, until the lock policy is promoted into workflow
  configuration.
- Backup `import` and `export --format json|markdown`, which are not canonical
  projection/rebuild.

Compatibility commands may be removed after their replacement path and migration
window are documented.

## Integration Or Experimental

Integration commands can remain implemented, but must not define the product's
default mental model:

- `cpitd`, because it depends on external clone-detection behavior and creates
  issues directly.
- `usage`, because token accounting belongs to future run/session or Mission
  Control policy.
- Assumption-specific impact commands such as `cascade` and `falsify`.

Generic replacements should use domain language. `atelier issue impact <id>` is
the visible relation-impact command. Until first-class `atelier link` commands
define directed typed-link semantics, impact follows hierarchy plus the
impact-bearing relation types `derived`, `caused-by`, and `falsifies`
transitively, and `assumption` one hop from the source. The inherited `cascade`
and `falsify` commands are removed so reassessment stays an explicit operator
decision through `issue impact`, `issue label`, `issue comment`, or `issue close`
instead of an assumption-specific command path.

## Removed Or Deferred Behavior

The daemon surface and changelog-on-close behavior are not part of the target
public workflow. Issue closure records close state, close time, and optional
reason in tracker state; it does not mutate `CHANGELOG.md`.

The inherited backup export/import path is preserved only as compatibility.
Canonical state is Markdown under `.atelier-state/`, checked with
`atelier export --check` and indexed into SQLite with `atelier rebuild`.
`atelier export` remains a compatibility, repair, and deterministic rendering
command during the Markdown-first migration; it is not the target step that
makes an otherwise successful mutation durable.
