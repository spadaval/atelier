# `atelier history`

Primary role: Reviewer.

Primary question: "What happened on this repo, mission, issue, or epic?"

## Assessment

- Name: Correct.
- Documentation: Correct. It should be the durable activity read surface, not
  local command telemetry.
- Design: Correct if output remains bounded and scoped.
- Output hierarchy: Scope and filters, newest activity, event kind/actor/time,
  drill-down commands.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `history` | Reviewer, manager/orchestrator | Inspect recent repository activity; recover context after interruption; audit who changed tracker state. | Source boundary, filters, newest events, actor, event kind, timestamp, target ID. | Add scope with `--issue`, `--mission`, or `--epic`; run `issue show` or `mission show`. | Strong fit as durable activity, not local command telemetry. |
| `history --mission <id>` | Reviewer, manager/orchestrator | Review mission proof trail; inspect coordination notes; see linked work activity. | Mission events plus linked-work activity, blocker/proof changes, actors, and times. | Run `mission status <id>`, `mission show <id>`, or inspect a child issue. | Good mission continuity surface. |
| `history --issue <id> [--include-descendants]` | Worker, reviewer, validator | Read handoff trail; inspect notes/evidence/review changes; continue interrupted work. | Issue activity, child activity when requested, proof/review events, actor chronology. | Run `issue show <id>`, `evidence show <id>`, or `review comments --issue <id>`. | Good. `--include-descendants` should stay explicit to avoid noisy defaults. |
| `history --epic <id>` | Reviewer, manager/orchestrator | Review epic branch activity and descendants; audit parent completion claims. | Epic and descendant events, proof trail, review/merge events. | Run `graph impact <id>`, `issue show <id>`, or `mission status`. | Good fit with epic as branch/review boundary. |

## Guidance Finding

No significant naming issue. Keep history bounded and scoped; it should help a
future operator reconstruct work without turning status or issue show into the
full log view.
