# `atelier branch`

Primary role: Manager/orchestrator.

Primary question: "How do I create, inspect, or merge the review branch for an
epic?"

## Assessment

- Name: Correct.
- Documentation: Correct in source help, but the installed `atelier` binary may
  lag and omit this command.
- Design: Correct. It keeps epic review boundaries separate from issue slices
  and mission worktrees.
- Output hierarchy: Epic ID, branch name, mission workspace, review/merge state,
  next status or merge command.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `branch for-epic <id>` | Manager/orchestrator | Create or switch to epic review branch. | Good. |
| `branch status` | Manager/orchestrator | Inspect local epic review branches. | Good. |
| `branch merge <id>` | Manager/orchestrator | Merge epic review branch. | Good. |
