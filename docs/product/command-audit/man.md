# `atelier man`

Primary roles: Worker and manager/orchestrator.

Primary questions: "Which existing Atelier commands matter for my current
role?" and "How does Atelier divide work among missions, epics, and issues?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| Which commands matter for my current role? | Worker | Low when guides are short and role-specific. | Small guidance renderer over current tracker state. | Keep | Keep exact roles and ranked loops; do not add role aliases or diagnostic detours. |

## Assessment

- Name: Correct. `man` is a guide layer, not a new command namespace.
- Documentation: Visible in root help and product docs as the role-specific
  replacement for the removed `prime` signpost and as the executable home for
  Atelier-owned work-model terminology.
- Design: Correct if valid roles are exact: `worker`, `reviewer`, `validator`,
  `manager`, and `admin`. Do not add `orchestrator` aliases.
- Output hierarchy: Role title, current state, ranked relevant commands, normal
  loop, then commands not usually for the role.
- Topic hierarchy: Concepts, graph shape, sizing, proof ownership, then current
  commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier man` | Worker | Discover valid role guides and product topics. | Good. |
| `atelier man worker` | Worker | Implement assigned or ready work. | Good. Requires valid tracker state. |
| `atelier man reviewer` | Reviewer | Check proof and workflow readiness. | Good. Requires valid tracker state. |
| `atelier man validator` | Validator | Run explicit validation and record validation proof. | Good. Requires valid tracker state. |
| `atelier man manager` | Manager/orchestrator | Coordinate missions, planning artifacts, blockers, and work. | Good. Requires valid tracker state. |
| `atelier man admin` | Admin | Set up, repair, migrate, and maintain Atelier. | Good. Degrades gracefully before init or broken state. |
| `atelier man work-model` | Manager/orchestrator | Choose and connect missions, epics, and issues. | Good. Static product guidance works before initialization. |

## Role Term

`manager` is the broad CLI role class for work coordination. `orchestrator` is a
specific Agent Factory agent type within that class, not a `man` role alias.

## Product Topic Boundary

`work-model` owns Atelier's mission, epic, and issue split. It explains that
missions scope direct `advances` roots plus descendants, epics own ordinary
child issues and the normal review boundary, and issues own local accountable
proof. This repository-specific vocabulary does not belong in portable Agent
Factory references.

## Human Output Debt

Role guides are readable, but they should participate in the same refresh:

- replace stale placeholders such as `<mission-id>` with `<objective-id>` where
  the command is really an issue objective command;
- color or de-emphasize role headings and current-state health only when the
  output is interactive;
- keep command lists short and ranked; and
- avoid teaching broad diagnostics or destructive commands unless the role or
  current state explicitly needs them.
