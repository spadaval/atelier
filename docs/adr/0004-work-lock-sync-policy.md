# ADR 0004: Work Association Replaces Default Lock Sync

## Status

Accepted.

## Context

Atelier inherits Chainlink lock and sync machinery. That machinery is useful for
some local coordination, but the target product now has canonical Markdown
records, rebuildable SQLite projection state, explicit work lifecycle commands,
and Git worktree helpers.

Normal agent workflow needs to know which issue is being worked, which branch or
worktree is associated with it, whether durable state is current, and whether a
transition is allowed. It does not need a remote lock-sync protocol as the
default coordination mechanism.

## Decision

Normal tracked work uses explicit work association, not inherited lock sync.

Root `atelier start`, root `atelier finish`, root `atelier status`, and
`atelier worktree for` own the default ergonomic path:

- record the issue, branch, and worktree association in local runtime state;
- keep Git as the source of truth for branches, commits, and worktrees;
- reject dirty source worktrees where the workflow action depends on a clean
  repository;
- check canonical export freshness before finish;
- never launch or supervise coding agents.

The hidden `atelier work status` helper and runtime-association internals are
implementation surfaces, not the normal operator workflow.

Inherited lock and sync commands are removed from the CLI surface. Internal
lock-checking helpers may remain only where core workflow code still needs
them. Remote/shared lock sync is deferred until a later policy explicitly
defines holder identity, expiry, conflict resolution, and Mission Control
projection semantics.

## Consequences

- Work lifecycle behavior stays understandable as Git plus local runtime
  association.
- A fresh worktree can rebuild projection state from tracked `.atelier/`
  records and then establish its own local work association.
- Advanced/manual coordination needs a new explicit core proposal rather than a
  hidden inherited command.
- Mission Control can project branch/worktree association separately from locks.
- Future lock redesign can build on the work association model without keeping
  inherited Chainlink sync behavior as the default.
