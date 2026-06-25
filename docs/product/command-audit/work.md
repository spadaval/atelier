# `atelier work`

Primary role: Worker and manager/orchestrator.

Primary question: "Which operational dashboard answers the next multi-issue
question?"

`work` is the dashboard namespace for multi-issue operational views. It does not
own issue mutation, workflow transitions, or durable record text. Those remain
under `issue`, `evidence`, `review`, and `history`.

## Dashboard Contract

Bare `atelier work` lists available dashboards:

- `work queue`: repo-wide operational queue with status, category, label,
  priority, ready, active, blocked, backlog, and all filters.
- `work mission <mission-id>`: live mission orchestration dashboard with health,
  progress, blockers, proof gaps, close readiness, and next actions.
- `work epic <epic-id>`: focused epic execution dashboard with child work,
  blockers, proof gaps, transition readiness, and next actions.

`work ready`, `work blocked`, `work active`, and `work all` remain short bucket
views for current role guidance, but new multi-issue behavior should be designed
under `work queue`.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `work` | Worker | Discover available dashboards. | Good. Bare output is guidance, not an implicit queue. |
| `work queue` | Worker | Browse repo-wide actionable work. | Good. Replaces removed `issue list` behavior. |
| `work queue --ready` | Worker | Choose selectable work. | Good. Quiet mode returns IDs only. |
| `work queue --blocked` | Manager/orchestrator | Inspect work with open blockers. | Good. Uses blocker state, not only workflow status category. |
| `work mission <id>` | Manager/orchestrator | Coordinate one live mission. | Good. Complements `issue show <mission-id>`. |
| `work epic <id>` | Worker/reviewer | Coordinate one epic boundary. | Good. Complements `issue show <epic-id>`. |

## Boundary

Panels render supplied view facts. They may group, bound, style, and label rows,
but they must not query storage, resolve IDs, run validators, or mutate tracker
state. Command adapters and app read models own those facts.

Color is semantic and optional. Colorless output must retain every state token,
blocker count, omitted-row count, and next command needed to act.

