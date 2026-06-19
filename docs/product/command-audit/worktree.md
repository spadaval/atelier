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
  stale warnings, next status or merge command.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `worktree for-mission <id>` | Manager/orchestrator | Create or locate the shared mission workspace; prepare background checkout for coordinated work. | Mission ID, path, branch, existing/new status, dirty/stale warnings. | Switch there and run `atelier mission status <id>`. | Good default workspace path. |
| `worktree for <issue-id>` | Manager/orchestrator | Create exceptional isolated issue workspace; separate risky work from mission workspace. | Issue ID, path, branch, parent mission/epic context, exceptional-use warning. | Work there, record proof, then merge/remove. | Help should say issue worktrees are exceptional, not the normal assignment model. |
| `worktree status` | Worker, manager/orchestrator | Inspect workspace state; find dirty/ahead/behind worktrees; recover ownership context. | Path, branch, dirty state, associated mission/issue, merge/remove readiness. | Commit/stash, inspect issue, merge, remove, or repair. | Good scan view if bounded. |
| `worktree merge <id>` | Manager/orchestrator | Manually merge associated work branch; recover when normal lifecycle integration needs help. | Source/target branch, merge result, conflicts, next validation. | Validate, run `mission status`, then remove worktree. | Acceptable advanced wrapper; recovery guidance should be explicit on conflicts. |
| `worktree remove <id> [--force]` | Manager/orchestrator | Clean up associated worktree after merge or abandoned isolation. | Removed path, dirty/refusal details, force consequence. | Run `worktree status`. | `--force` risk should be explained in help/output. |
| `worktree repair <id>` | Admin | Clear stale local association after interrupted setup/removal. | Stale association, ignored runtime state touched, confirmation. | Run `worktree status` or retry remove/create. | Good runtime repair command; distinguish from removed root `repair`. |

## Guidance Finding

Mission worktrees are the normal coordination boundary; per-issue worktrees are
exceptional. Help and role guides should make that distinction visible so agents
do not create a new checkout for every issue.
