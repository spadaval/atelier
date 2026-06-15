# `atelier mission`

Primary role: Manager/orchestrator.

Primary question: "How do I create, focus, inspect, coordinate, and close a
durable mission?"

## Assessment

- Name: Correct. Mission is the durable purpose boundary.
- Documentation: Strong, but role guides should distinguish daily mission
  status from closeout audit and lifecycle mutation.
- Design: Mostly correct. The family owns mission purpose, focus, linked work,
  blockers, and closeout.
- Output hierarchy: Mission identity and lifecycle first, current work/blockers
  next, proof/health/closeout readiness next, then specific next actions.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `mission create` | Manager/orchestrator | Create mission purpose, constraints, risks, validation criteria. | Good. |
| `mission show` | Manager/orchestrator | Inspect rich mission state and linked records. | Good. |
| `mission start --switch` | Manager/orchestrator | Set active mission focus. | Good. |
| `mission status` | Manager/orchestrator | See current mission health and next actions. | Good. Also useful to workers. |
| `mission status --closeout --verbose` | Reviewer | Inspect closeout readiness details. | Good if verbose detail remains secondary. |
| `mission audit` | Reviewer | Drill into closeout proof and approval gaps. | Needs framing as special closeout drill-down, not daily status. |
| `mission close --reason` | Manager/orchestrator | Close a mission after gates pass. | Good. |
| `mission list` | Manager/orchestrator | Select current or historical missions. | Good. |
| `mission update` | Manager/orchestrator | Change lifecycle fields and mission sections. | Good, but docs should discourage `--status closed` as ordinary closeout. |
| `mission note` | Manager/orchestrator | Add durable coordination or handoff context. | Good. |
| `mission add-work` | Manager/orchestrator | Link issue work into mission scope. | Good. |
| `mission unlink` | Manager/orchestrator | Remove issue work from mission scope. | Good. |
| `mission add-blocker` | Manager/orchestrator | Mark an issue as a mission blocker. | Good. |
