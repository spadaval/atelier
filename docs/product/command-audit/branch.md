# `atelier branch`

Primary role: Advanced manager/orchestrator recovery.

Primary question: "How do I inspect or recover an owner branch when normal
workflow transition effects cannot complete automatically?"

## Assessment

- Name: Correct.
- Documentation: Correct in source help. It belongs in advanced recovery
  guidance, not routine worker loops.
- Design: Advanced. Routine worker and manager loops should use
  `atelier issue transition` for lifecycle movement; transition effects own
  branch preparation and integration. Branch commands remain explicit recovery
  and diagnostic surfaces.
- Output hierarchy: Epic ID, branch name, mission workspace, review/merge state,
  next status or merge command.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `branch for-epic <id>` | Advanced manager/orchestrator recovery | Recover or create epic review branch manually; inspect branch ownership when transition effects cannot prepare it. | Epic ID, mission worktree, branch name, base/current branch, review state. | Run `branch status`, review/open artifact, or return to workflow transition. | Correctly advanced; normal flows should use lifecycle commands/effects. |
| `branch status` | Advanced manager/orchestrator recovery | Inspect local epic review branches; diagnose branch/worktree mismatch. | Mission worktree, current branch, epic branches, dirty/merge cues. | Run `branch merge <id>` or return to `mission status`. | Good diagnostic; subhelp could say mission-worktree-only if that is required. |
| `branch merge <id>` | Advanced manager/orchestrator recovery | Manually integrate epic branch; recover failed lifecycle integration. | Merge source/target, conflicts, result, remaining validation. | Validate, run `mission status`, then continue closeout. | Runtime guards are important; help is terse. |

## Guidance Finding

Branch commands should stay out of ordinary worker loops. They are recovery
surfaces for epic branch ownership when workflow-declared effects or review
integration cannot complete automatically.
