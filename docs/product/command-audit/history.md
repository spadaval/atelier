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

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `history` | Reviewer | Inspect recent repository activity. | Good. |
| `history --mission <id>` | Reviewer | Review mission activity and proof trail. | Good. |
| `history --issue <id>` | Reviewer | Review issue activity and handoff trail. | Good. |
| `history --epic <id>` | Reviewer | Review epic activity and descendants. | Good. |
