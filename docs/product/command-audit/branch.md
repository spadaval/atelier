# `atelier branch`

Primary role: Advanced manager/orchestrator recovery.

Primary question: "How do I inspect or recover an owner branch when normal
start or close lifecycle commands cannot complete automatically?"

## Assessment

- Name: Correct.
- Documentation: Correct in source help. It belongs in advanced recovery
  guidance, not routine worker loops.
- Design: Advanced. Routine worker and manager loops should use
  `atelier issue transition <id> start` for branch preparation and close
  lifecycle commands for integration; branch commands remain explicit recovery
  and diagnostic surfaces.
- Output hierarchy: Epic ID, branch name, mission workspace, review/merge state,
  next status or merge command.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `branch for-epic <id>` | Advanced manager/orchestrator recovery | Manually create or switch to an epic review branch when lifecycle-owned start cannot prepare it. | Advanced repair. |
| `branch status` | Advanced manager/orchestrator recovery | Inspect local epic review branches. | Advanced diagnostic. |
| `branch merge <id>` | Advanced manager/orchestrator recovery | Manually merge an epic review branch when close lifecycle integration requires recovery. | Advanced repair. |
