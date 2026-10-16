# ADR 0021: Parent Issues Replace Epics

## Status

Accepted (2026-10-15, decided by Sumeet in atelier-9vq3).

Partially supersedes [ADR 0018](0018-strategy-draft-epics-and-adaptive-expansion.md):
draft epics become `backlog` work items, and later increments stay in
`backlog` until their predecessor's evidence arrives.

## Context

Atelier's epic meant "a unit of work that goes through the lifecycle
together": it owned one branch and merged only when every child was done. Most
trackers use "epic" for a theme, such as "Trade". Adopters modelled themes as
epics and got branches that never merged; astromech split its backlog into 19
increments to avoid that. An epic's Outcome, Proof and Delivers map onto an
issue's Goal and Done when plus its children, so the separate type mostly added
a word that means something else elsewhere, and special cases: children of
draft epics, an epic's blockers versus its children's, and epics in the ready
list.

## Decision

1. **No epic type.** Any non-mission work item may have direct children, one
   level deep: a child has no children of its own, and missions are never
   parents.
2. **The parent owns the branch.** Children work on their parent's branch
   (`<issue_type>/<parent-id>`), and the parent closes only after every child
   is terminal. The starter policy's `close` runs `work.terminal` for this.
   Superseding a parent retires it with any open children.
3. **One body shape for work.** Work items have Goal and Done when, with
   optional Scope, Context, Unknowns, and Design. Missions keep Outcome and
   Success criteria; the mission `Epics` section is gone.
4. **Themes are labels; missions keep the why.** Planned-but-not-near-term work
   sits in the `backlog` status.
5. **Migration.** `atelier migrate epics --apply` retypes epic records (as
   `feature` by default), maps Outcome to Goal and Proof and Delivers to Done
   when, moves draft epics to `backlog`, keeps their links, removes `epic`
   from the policy, and adds `work.terminal` to a starter-shaped work `close`. Records with the old shape fail to parse with a pointer to
   the command.

## Consequences

- `atelier work epic` becomes `atelier work parent`, and the `epic/<id>`
  evidence target alias is gone; use `issue/<id>`.
- Adopters with epic records run the migration once and review the Done when
  sections it writes.
- One parent rule covers the remaining special cases, which atelier-e4c5
  (a parent's blockers apply to its children), atelier-ymmb (parents are
  context in the ready list), and atelier-28mz (children of backlog parents)
  build on.
- Agent Factory's skill still says "epic"; per ADR 0020 that changes upstream
  (atelier-92mz).
