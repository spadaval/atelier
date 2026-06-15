# ADR 0004: Work Association Replaces Default Lock Sync

## Status

Accepted. Amended by
[ADR 0007](0007-mission-workspaces-and-epic-review-branches.md), which moves
the default workspace boundary from issue to mission and the default review
boundary from issue to epic, and by
[ADR 0008](0008-layered-cargo-workspace.md), which treats runtime active-work
and claim tables as migration residue rather than a durable current-work source
of truth.

## Context

Atelier inherits Chainlink lock and sync machinery. That machinery is useful for
some local coordination, but the target product now has canonical Markdown
records, rebuildable SQLite projection state, explicit work lifecycle commands,
and Git worktree helpers.

Normal agent workflow needs to know which issue slice is being worked, which
mission worktree and epic branch are associated with it, whether durable state
is current, and whether a transition is allowed. It does not need a remote
lock-sync protocol as the default coordination mechanism.

## Decision

Normal tracked work uses explicit workflow state and checkout context, not
inherited lock sync or runtime-only active-work state.

Root `atelier start`, `atelier issue close`, root `atelier status`,
`atelier worktree for-mission`, and `atelier branch for-epic` own the default
ergonomic path:

- derive current work from canonical issue status, mission/epic graph links, and
  Git checkout state;
- optionally record issue slice, epic branch, mission worktree, and session
  association in local runtime state as rebuildable ergonomic hints;
- keep Git as the source of truth for branches, commits, and worktrees;
- reject dirty source worktrees where the workflow action depends on a clean
  repository;
- check canonical export freshness before closeout;
- never launch or supervise coding agents.

Runtime active-work, session, and claim rows are not the source of truth for
what work is current. Losing or rebuilding `.atelier/runtime/state.db` may remove
local ergonomic hints, but it must not change canonical issue status, readiness,
mission membership, blocker state, or closeout eligibility. Hidden
`atelier work status` helpers and runtime-association internals are
implementation surfaces, not the normal operator workflow and not public
compatibility promises.

Inherited lock and sync commands are removed from the CLI surface. Internal
lock-checking helpers may remain only where core workflow code still needs
them. Remote/shared lock sync is deferred until a later policy explicitly
defines holder identity, expiry, conflict resolution, and Mission Control
projection semantics.

## Consequences

- Work lifecycle behavior stays understandable as Git plus local runtime
  association hints plus canonical issue workflow state.
- A fresh mission worktree can rebuild projection state from tracked
  `.atelier/` records and then establish its own local work association hints.
- Advanced/manual coordination needs a new explicit core proposal rather than a
  hidden inherited command.
- Mission Control can project branch/worktree association separately from locks.
- Future lock redesign can build on the work association model without keeping
  inherited Chainlink sync behavior as the default.
- Active-work and claim removal work should delete runtime-source-of-truth
  behavior directly. Temporary readers or adapters must follow the internal
  migration adapter policy in ADR 0008 and carry an owner, removal condition,
  and no public compatibility promise.
