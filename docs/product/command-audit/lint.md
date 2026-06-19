# `atelier lint`

Primary role: Reviewer.

Primary question: "Are committed tracker records and workflow configuration
valid?"

## Assessment

- Name: Correct.
- Documentation: Correct. It should be in reviewer and admin guides.
- Design: Correct if it validates canonical state and does not silently repair
  tracked records.
- Output hierarchy: Pass/fail summary, offending record/config path, actionable
  repair guidance, next `doctor` only when runtime state is implicated.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `lint` | Reviewer, admin, validator | Validate committed tracker records before handoff; diagnose workflow/config errors; check repo-wide canonical state. | Pass/fail summary, file path, field path, message, and whether the problem is tracked canonical state. | Fix named Markdown/config, rerun `atelier lint`, then return to `status` or `mission status`. | Strong fit. In this checkout it correctly reports `.atelier/workflow.yaml` `workflows.standard.transitions.start.effects` as invalid for the installed binary. |
| `lint <id>` | Reviewer, worker | Validate a focused issue or imported source ID; check one record before close/handoff. | Focused record status, parse/validation failures, exact path/field. | Fix the record, rerun focused lint, then continue transition or evidence work. | Good focused gate. |

## Guidance Finding

`lint` owns committed-state validity. It should route to `doctor` only when the
problem is local runtime/projection state; tracked Markdown and workflow config
must be fixed as repository artifacts.
