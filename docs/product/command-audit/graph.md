# `atelier graph`

Primary role: Manager/orchestrator.

Primary question: "What relationship context is missing from issue and mission
views?"

## Assessment

- Name: Weak. `graph` is implementation language; operators usually start from
  an issue, mission, blocker, or proof question.
- Documentation: Should mark this namespace as a consolidation target rather
  than teaching it as an ordinary manager path.
- Design: Duplicative. `graph tree` overlaps with mission and issue hierarchy
  views. `graph impact` exposes useful blast-radius logic, but that is evidence
  that `issue show` and `mission show` do not yet expose enough relationship
  context.
- Output hierarchy: If any implementation survives temporarily, it should
  report the source record, relationship reason, affected record, and the
  domain command that should own the next drill-down.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `graph impact <id>` | Manager/orchestrator | Understand downstream consequences before mutation or terminal validation. | Consolidate. Fold issue-sourced impact into `issue show`; fold mission-sourced impact into `mission show` or `mission status`. Reconsider a domain command only after those views are stronger. |
| `graph tree [--compact]` | Manager/orchestrator | Inspect mission and issue hierarchy. | Remove or fold into `mission show` and scoped issue hierarchy output. |
| `graph tree --status <status>` | Manager/orchestrator | Filter the hierarchy by status. | Remove. It is redundant with issue list/status/category filters and currently hard-codes predecessor statuses. |

## Cutting Direction

Delete the abstract `graph` namespace unless a post-consolidation need remains.
The first implementation step should strengthen `issue show` and `mission show`
with relationship sections: parent/children, blockers, mission links, evidence
links, and affected records that should be reviewed before changing or closing
the source. A compact blast-radius view can return later as `issue impact` or
`mission impact` only if the richer show views still leave a real operator
question unanswered.
