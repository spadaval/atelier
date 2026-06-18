# `atelier man`

Primary role: Worker.

Primary question: "Which existing Atelier commands matter for my current role?"

## Assessment

- Name: Correct. `man` is a guide layer, not a new command namespace.
- Documentation: Should be visible in root help and product docs as the
  role-specific replacement for the removed `prime` signpost.
- Design: Correct if valid roles are exact: `worker`, `reviewer`, `validator`,
  `manager`, and `admin`. Do not add `orchestrator` aliases.
- Output hierarchy: Role title, current state, ranked relevant commands, normal
  loop, then commands not usually for the role.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier man` | Worker | Discover valid role guides. | Good. |
| `atelier man worker` | Worker | Implement assigned or ready work. | Good. Requires valid tracker state. |
| `atelier man reviewer` | Reviewer | Check proof and workflow readiness. | Good. Requires valid tracker state. |
| `atelier man validator` | Validator | Run explicit validation and record validation proof. | Good. Requires valid tracker state. |
| `atelier man manager` | Manager/orchestrator | Coordinate missions, planning artifacts, blockers, and work. | Good. Requires valid tracker state. |
| `atelier man admin` | Admin | Set up, repair, migrate, and maintain Atelier. | Good. Degrades gracefully before init or broken state. |

## Role Term

`manager` is the broad CLI role class for work coordination. `orchestrator` is a
specific Agent Factory agent type within that class, not a `man` role alias.
