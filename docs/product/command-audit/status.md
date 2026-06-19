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

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `atelier status` | Worker, all roles | Orient to current work; check active mission; see stale/blocked/missing-proof warnings. | Current `in_progress` issue set, active mission, ready/blocked counts, branch/worktree safety, evidence gaps, and one or two next commands. | Run `atelier issue transition <issue-id> start`, `atelier issue show <id>`, `atelier mission status <id>`, `atelier issue transition <id> --options`, or `atelier doctor --fix`. | Correct first command, but must degrade when tracker state is invalid instead of failing before orientation. |

## Guidance Finding

In this checkout, `atelier status` fails before rendering any orientation because
workflow policy parsing rejects `.atelier/workflow.yaml`
`workflows.standard.transitions.start.effects`. That blocks the command that is
supposed to answer "what should I do next?" at exactly the friction point where
`zen.md` says guidance matters. The ideal degraded output should name the
invalid config path and route to `atelier lint` or `atelier man admin`, while
keeping normal status compact enough that it does not become the full record
view.
