# ADR 0004: Work Association Replaces Default Lock Sync

## Status

Accepted. Amended by
[ADR 0007](0007-mission-workspaces-and-epic-review-branches.md), which moves
the default workspace boundary from issue to mission and the default review
boundary from issue to epic. Superseded for current-work source-of-truth by the
crate migration contract: current work is derived from canonical issue workflow
status in the checkout's tracked Markdown records, not from ignored runtime
work associations.

## Context

Atelier inherits Chainlink lock and sync machinery. That machinery is useful for
some local coordination, but the target product now has canonical Markdown
records, rebuildable SQLite projection state, explicit work lifecycle commands,
and Git worktree helpers.

Normal agent workflow needs to know which issue slices are in progress in the
current checkout, which mission worktree and epic branch contain the changes,
whether durable state is current, and whether a transition is allowed. It does
not need a remote lock-sync protocol as the default coordination mechanism.

## Decision

Normal tracked work uses canonical issue workflow status and checkout context,
not inherited lock sync and not ignored runtime active-work association.

Root `atelier start`, `atelier issue close`, root `atelier status`, and
`atelier worktree for-mission` own the default ergonomic path:

- record current work by moving issue records through workflow status, with
  `in_progress` records forming the current-work set for that tracker copy;
- keep Git as the source of truth for branches, commits, and worktrees;
- reject dirty source worktrees where the workflow action depends on a clean
  repository;
- check canonical tracker health and local projection health through
  `lint`/`doctor` before closeout;
- never launch or supervise coding agents.

Explicit branch helpers are advanced diagnostics and repair surfaces. They do
not own routine branch preparation, which belongs to `atelier start`.

Ignored runtime tables may cache diagnostics or projection state, but runtime
`work_associations`, hidden claims, sessions, and active pointers are not
durable current-work source-of-truth surfaces. A fresh checkout can rebuild the
projection and recover current-work orientation from tracked `.atelier/`
records plus Git context.

Inherited lock and sync commands are removed from the CLI surface. Internal
lock-checking helpers may remain only where core workflow code still needs
them. Remote/shared lock sync is deferred until a later policy explicitly
defines holder identity, expiry, conflict resolution, and Mission Control
projection semantics.

## Consequences

- Work lifecycle behavior stays understandable as Git plus canonical issue
  workflow status.
- A fresh mission worktree can rebuild projection state from tracked
  `.atelier/` records and recover its current-work set without hidden runtime
  associations.
- Advanced/manual coordination needs a new explicit core proposal rather than a
  hidden inherited command.
- Mission Control can project branch/worktree orientation separately from locks.
- Future lock redesign can build on canonical workflow status and Git context
  without keeping inherited Chainlink sync behavior as the default.
