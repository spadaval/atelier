# Retired `atelier worktree`

Primary role: Retired/deferred workspace management.

Primary question: "How do I prepare, inspect, merge, repair, or remove
mission/issue worktrees?"

## Assessment

- Name: Too broad for the current product. It exposed Git workspace mechanics
  instead of strengthening the issue, mission, status, and branch views.
- Documentation: Retire from normal role guides and root help. Keep this audit
  page only as a record of the cut.
- Design: Remove pending redesign. Mission worktree ownership and per-issue
  worktree setup were buggy, under-specified, and not valuable enough to carry
  as first-class surface.
- Replacement: Use `atelier status`, `atelier issue status <id>`,
  `atelier issue transition <id>`, and workflow-backed issue
  transitions for ordinary work. Use `atelier branch ...` only for advanced
  owner-branch recovery.

## Retired Subcommands

| Form | Previous role | Replacement |
| --- | --- | --- |
| `worktree for-mission <id>` | Manager/orchestrator | Deferred. Use normal checkout plus `issue status <objective-id>` until workspace management is redesigned. |
| `worktree for <issue-id>` | Manager/orchestrator | Deferred. Use issue transitions and explicit branch/checkouts outside Atelier when isolation is required. |
| `worktree status` | Worker/manager | `atelier status`, `git status --short --branch`, and `issue transition <id> transition options`. |
| `worktree merge <id>` | Manager/orchestrator | Workflow close transitions or advanced `branch merge` for epic-owner recovery. |
| `worktree remove <id>` | Manager/orchestrator | Plain Git cleanup outside Atelier until workspace management returns with a clearer contract. |
| `worktree repair <id>` | Admin | `doctor --fix` for ignored runtime/projection state; canonical issue transitions for durable workflow state. |
