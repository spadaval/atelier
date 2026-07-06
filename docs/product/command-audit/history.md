# `atelier history`

Primary role: Reviewer.

Primary question: "What happened on this repo, mission, issue, or epic?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| What durable activity matters for this scope? | Reviewer | Low when timelines are bounded and scope is singular. | Canonical activity reads remain simple without graph/query expansion. | Simplify | Keep concise root and one-record history; fold objective and filter queries into their existing owners. |

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
| `history` | Reviewer | Inspect recent repository activity. | Keep, bounded to 20 events by default. |
| `history --issue <id>` | Reviewer | Review one issue-shaped record's activity and linked proof trail. | Keep. It provides more activity than bounded `issue show` without traversing descendants. |
| `history --mission <id>` / `--epic <id>` | Reviewer | Review objective descendants. | Fold into `work mission`, `work epic`, and `issue show`; the old flags are removed. |
| `--include-descendants`, `--event-kind`, `--actor`, `--since` | Reviewer | Build a scoped activity query. | Remove. These flags made history a second query language. |

## Complexity Budget

`history` should remain a bounded activity reader, not a second search or query
language. The root timeline can stay if it is concise. Scoped issue, mission,
and epic history flags need an explicit value check against recent activity in
`issue show`, `work mission`, and validation/evidence records.

Verdict: keep repository history, keep one-record `--issue` history, simplify
breadth to `--limit`, fold mission/epic descendant questions into objective
views, and remove filter/descendant query flags.

## Human Output Contract

History renders a compact event sentence first and de-emphasizes timestamp,
event kind, actor, and target on a second line. Scope, newest-first ordering,
the 20-event default limit, and omitted counts remain visible. Quiet output
contains only the total event count and bounded timestamps.
