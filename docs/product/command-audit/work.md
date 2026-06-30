# `atelier work`

Primary role: Manager/orchestrator for scoped coordination, worker only for
explicit ready-work pickup.

Primary question: "Which bounded work view reduces the next coordination
decision?"

`work` is the dashboard namespace for multi-issue operational views. It does not
own issue mutation, workflow transitions, or durable record text. Those remain
under `issue`, `evidence`, `review`, and `history`.

The namespace must not become a junk drawer. Each visible `work` command needs a
role, a normal decision point, and a smaller cognitive burden than the operator
would have from `issue list`, `issue show`, and `issue transition`.

## Dashboard Contract

Bare `atelier work` lists available dashboards. It is an orientation surface,
not a data view.

- `work ready`: small picker for top-level work that can be started or
  coordinated now.
- `work blocked`: manager triage for work stopped by open blockers.
- `work active`: in-flight work surface when the operator needs to see what is
  already moving.
- `work queue`: unresolved pressure point. If kept, it must be a bounded
  repo-wide operational overview, not generic inventory and not a replacement
  for scoped dashboards.
- `work mission <mission-id>`: live mission orchestration dashboard with
  mission-scoped progress, ready/active/blocked/done workstreams, blockers,
  closeout only when relevant, and next actions.
- `work epic <epic-id>`: focused epic execution dashboard with child work,
  blockers, proof gaps, transition readiness, and next actions.

`work ready`, `work blocked`, and `work active` are useful only if they lower
cognitive complexity more than a single broader queue would. They should stay
short, role-shaped, and boring. If they grow flags or hierarchy, that is a sign
the behavior belongs in `work mission`, `work epic`, or `issue list`.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `work` | Worker | Discover available dashboards. | Good. Bare output is guidance, not an implicit queue. |
| `work ready` | Manager/worker | Pick the next top-level objective or explicitly unassigned item. | Keep. This is the normal small entry point. |
| `work blocked` | Manager/orchestrator | Triage blocked work across the repo. | Keep if terse. It has a distinct interruption/removal job. |
| `work active` | Manager/orchestrator | See work already in motion. | Keep only if it answers in-flight coordination better than `status`. |
| `work all` | Manager/orchestrator | Inspect all operational buckets at once. | Questionable. High cognitive load; prefer scoped dashboards unless a clear all-buckets job is proven. |
| `work queue` | Unclear | Browse repo-wide actionable work. | Simplify or fold. Current output is a nested repo-wide dump that overlaps `work ready`, `work blocked`, `work active`, `work mission`, `work epic`, and `issue list`. |
| `work queue --ready` | Worker/scripts | Choose selectable leaf work. | Fold toward `work ready` unless quiet leaf IDs are a proven automation need. |
| `work queue --blocked` | Manager/orchestrator | Inspect work with open blockers. | Fold toward `work blocked` unless the broader queue adds distinct context. |
| `work mission <id>` | Manager/orchestrator | Coordinate one live mission. | Keep. It avoids stitching issue detail, blockers, and child state across commands. |
| `work epic <id>` | Worker/reviewer | Coordinate one epic boundary. | Keep only if it remains tighter than `issue show <epic-id>` plus child issue drill-down. |

## Boundary

Panels render supplied view facts. They may group, bound, style, and label rows,
but they must not query storage, resolve IDs, run validators, or mutate tracker
state. Command adapters and app read models own those facts.

Color is semantic and optional. Colorless output must retain every state token,
blocker count, omitted-row count, and next command needed to act.

## Complexity Budget

`work` must not become a second query language. Keep broad issue inventory in
`issue list`, record detail in `issue show`, and lifecycle gates in
`issue transition`.

The current `work queue` identity is too broad. Its output shows repo-wide
mission, epic, task, blocker, validation, and standalone context in one view.
That increases product complexity because the operator has to decide whether
they are choosing a mission, selecting a leaf task, triaging blockers, or
auditing backlog shape. It also risks architecture complexity because every
extra queue flag tempts the command to duplicate inventory, dashboard, blocker,
and hierarchy read models.

The fix is not automatically "more purpose-built views." A purpose-built view
survives only when it removes command stitching for a real role decision:

- `work ready` answers the small picker question.
- `work blocked` answers the interruption triage question.
- `work mission <id>` answers the mission coordination question.
- `work epic <id>` answers the epic execution-boundary question.
- `issue list` answers the inventory question.

Anything left for `work queue` must be named explicitly. If no distinct
operator question remains, remove it from normal guidance and fold its useful
behavior into the commands above.

`work mission` should be epic-first by default. Child tasks appear when they are
active, blocked, or specifically requested by a scoped drill-down flag. Default
ready work should not be a random flat list of leaf tasks.
