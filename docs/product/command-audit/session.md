# `atelier session`

Primary role: Reviewer.

Primary question: "What issue-scoped worker, reviewer, or validator attempts can
I inspect from canonical activity?"

## Assessment

- Name: Acceptable. `session` is familiar, but product docs should keep saying
  sessions are derived inspection views, not workflow records.
- Documentation: Needs orientation to prevent agents from treating sessions as
  current work or assignment state.
- Design: Correct if read-only. It supports attribution and continuity without
  making one run equal one issue.
- Output hierarchy: Attempt ID, issue ID, role, lifecycle, actor/time, relevant
  notes/evidence/review events, then issue/history drill-down.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `session list [--active]` | Reviewer, manager/orchestrator | See active attempts; audit who touched an issue; find handoff candidates after interruption. | Attempt IDs, issue IDs, role, state, actor, last activity, and whether the issue itself is still active/blocked/done. | Run `atelier session show <attempt-id>` or `atelier issue show <issue-id>`. | Needs explicit "inspection-only" wording in docs/help; current work remains issue workflow status. |
| `session show <id>` | Reviewer, validator | Inspect a specific attempt; reconstruct handoff context; check whether proof came from a worker, reviewer, or validator attempt. | Ordered activity, role, producer, linked evidence/review events, and residual risks. | Run `atelier history --issue <id>`, `atelier evidence show <id>`, or `atelier issue transition <id> --options`. | Should not point to session mutation because there is none. Good fit with continuity over private context. |

## Guidance Finding

`session` should always orient back to accountable issue records. The next action
is inspect or validate the issue, not "end" or "close" a session.
