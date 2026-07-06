# `atelier work`

Primary role: Manager/orchestrator for scoped coordination, worker only for
explicit ready-work pickup.

Primary question: "Which bounded work view reduces the next coordination
decision?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| Which bounded view resolves my next coordination decision? | Manager/orchestrator; worker for explicit pickup | Scoped dashboards reduce scanning; the legacy queue mixes unrelated decisions. | Each view risks duplicating inventory, hierarchy, and blocker reads. | Simplify | Keep role-shaped views; leave `work queue` legacy and out of normal guidance. |

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
- `work missions`: the cross-mission Mission Overview. It shows current
  missions and directly advanced epics with collapsed descendant state, plus
  direct and outside-visible-mission summaries.
- `work queue`: legacy repo-wide output. It owns neither inventory nor Mission
  Overview and should leave normal guidance once its remaining callers move.
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
| `work missions` | Manager/orchestrator | Compare current missions and their directly advanced epics. | Keep as the bounded, epic-first Mission Overview; use `--all` to include done missions. |
| `work queue` | Legacy only | Browse the old repo-wide nested dump. | Retire from normal guidance and migrate remaining callers; do not alias it to inventory or Mission Overview. |
| `work queue --ready` | Legacy only | Choose selectable leaf work. | Retired from normal guidance. Use `work ready`. |
| `work queue --blocked` | Legacy only | Inspect work with open blockers. | Retired from normal guidance. Use `work blocked`. |
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

The legacy `work queue` identity is too broad. Its output shows repo-wide
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

No distinct operator question remains for `work queue` after the split. Remove
it from normal guidance and migrate useful behavior to the commands above
without a compatibility alias or fallback renderer.

`work mission` should be epic-first by default. Child tasks appear when they are
active, blocked, or specifically requested by a scoped drill-down flag. Default
ready work should not be a random flat list of leaf tasks.

The plural `work missions` is also epic-first, but more collapsed: directly
advanced epic rows are visible, leaf tasks never expand in the default, and
direct non-epic roots are summarized rather than presented as epic children.
See [Issue Inventory And Mission Overview](../issue-inventory-and-mission-overview.md)
for membership, shared work, done inclusion, exceptional-work accounting,
ordering, budgets, quiet output, and color behavior.
