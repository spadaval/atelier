# `atelier mission`

Primary role: Manager/orchestrator.

Primary question: "How do I create, focus, inspect, coordinate, and close a
durable mission?"

## Assessment

- Name: Correct. Mission is the durable purpose boundary.
- Documentation: Strong, but role guides should distinguish daily mission
  status from terminal-check detail and lifecycle mutation.
- Design: Mostly correct. The family owns mission purpose, focus, linked work,
  blockers, and terminal checks. It should also own mission-scoped hierarchy and
  blast-radius context now exposed through `graph tree` and mission-sourced
  `graph impact`.
- Output hierarchy: Mission identity and lifecycle first, current work/blockers
  next, linked hierarchy and affected records next, proof/health/terminal
  readiness next, then specific next actions.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `mission create` | Manager/orchestrator | Create mission purpose, constraints, risks, validation criteria. | Good. |
| `mission show` | Manager/orchestrator | Inspect rich mission state, linked records, hierarchy, and relationship context. | Needs strengthening. It should absorb mission-scoped `graph tree` and `graph impact` value. |
| `mission start --switch` | Manager/orchestrator | Set active mission focus. | Good. |
| `mission status` | Manager/orchestrator | See current mission health and next actions. | Good. Also useful to workers; should link to richer mission show output rather than graph helpers. |
| `mission status --verbose` | Reviewer | Inspect terminal-check detail. | Good if verbose detail remains secondary. |
| `mission close --reason` | Manager/orchestrator | Close a mission after gates pass. | Good. |
| `mission list` | Manager/orchestrator | Select current or historical missions. | Good. |
| `mission update` | Manager/orchestrator | Change lifecycle fields and mission sections. | Good, but docs should discourage `--status closed` as the ordinary terminal path. |
| `mission note` | Manager/orchestrator | Add durable coordination or handoff context. | Good. |
| `mission add-work` | Manager/orchestrator | Link issue work into mission scope. | Good. |
| `mission unlink` | Manager/orchestrator | Remove issue work from mission scope. | Good. |
| `mission add-blocker` | Manager/orchestrator | Mark an issue as a mission blocker. | Good. |
