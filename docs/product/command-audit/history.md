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

## Complexity Budget

`history` should remain a bounded activity reader, not a second search or query
language. The root timeline can stay if it is concise. Scoped issue, mission,
and epic history flags need an explicit value check against recent activity in
`issue show`, `work mission`, and validation/evidence records.

Verdict for scoped flags: simplify or fold unless full activity trails prove
necessary for review.

## Human Output Debt

Current history rows are dense pipe-delimited transcripts. They preserve the
facts, but humans have to parse timestamp, event kind, actor, target, title, and
summary from one long line.

Refresh target:

- group or wrap events so the event sentence is the primary text;
- de-emphasize repeated scope, actor, target, and record title metadata;
- keep filters and omitted counts visible;
- reserve raw activity fields for focused detail or verbose output; and
- keep the default limit bounded with an obvious command to broaden the view.
