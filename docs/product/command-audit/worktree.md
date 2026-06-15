# `atelier worktree`

Primary role: Manager/orchestrator.

Primary question: "How do I prepare, inspect, merge, repair, or remove
mission/issue worktrees?"

## Assessment

- Name: Correct.
- Documentation: Needs role clarity. Workers may inspect status, but managers
  usually decide workspace setup and cleanup.
- Design: Correct if mission worktrees are the default and issue worktrees are
  explicitly exceptional isolation.
- Output hierarchy: Mission/issue ID, path, branch/worktree state, dirty or
  stale warnings, current `in_progress` issue count from the checked-out
  tracker copy when useful, next status or merge command.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `worktree for-mission <id>` | Manager/orchestrator | Create or locate the shared mission workspace. | Good. |
| `worktree for <issue-id>` | Manager/orchestrator | Create or locate exceptional issue isolation. | Good if docs say exceptional. |
| `worktree status` | Worker | Inspect local workspace state. | Good. Also useful to managers. |
| `worktree merge <id>` | Manager/orchestrator | Merge associated work branch. | Good. |
| `worktree remove <id>` | Manager/orchestrator | Remove associated worktree. | Good, with force warning. |
| `worktree repair <id>` | Admin | Repair or remove stale Git worktree metadata when available. | Keep distinct from removed root active-pointer repair; it must not decide current work. |
