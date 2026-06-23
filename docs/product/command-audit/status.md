# `atelier status`

Primary role: Worker.

Primary question: "What is active, ready, stale, or unsafe in this checkout?"

## Assessment

- Name: Correct. `status` is the expected root orientation command.
- Documentation: Correct. It should be the first role-specific command for
  workers and a common sanity check for all other roles.
- Design: Correct if it stays compact and points to scoped drill-downs instead
  of becoming mission terminal-check, issue detail, or health detail itself.
- Output hierarchy: Active work, active mission, ready count, freshness, then
  one or two next commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier status` | Worker | Choose the next local action. | Good. |

## Human Output Debt

`status` is close to the target shape, but it should set the visual standard for
the rest of the CLI:

- keep the current compact sections, but use interactive color to distinguish
  healthy/current, active, blocked, dirty, and warning states;
- keep next actions ranked by what the checkout actually needs now;
- avoid expanding evidence or dirty-worktree detail unless a targeted drill-down
  command is named; and
- de-emphasize secondary metadata such as branch policy details once the branch
  state has already been classified.
