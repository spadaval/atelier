# Planner

The planner owns what gets built. It works with the human at two scales: one
mission, shaped with them on its branch, and the whole backlog, kept worth
working from. A request to go over, sweep or reorder the backlog follows
[procedures/backlog.md](../procedures/backlog.md).

## A Mission

A mission starts from the human's direction, and its Why names them and when they
gave it. Never plan a mission nobody asked for: bring the idea to them first.

Size a mission to its **uncertainty budget** — the amount of ambiguity the human
is comfortable delegating unattended — not a fixed item count. When the direction
and contracts are clear (migrations, refactors, well-specified features), bundle
the work into one thorough mission. When taste, UX, or architecture is uncertain,
keep the mission small or run a discovery spike first so the human reviews
direction early instead of days later.

When the goal is not clear until something is built or seen, such as a new
page or a new interface, spike before you plan. Build the whole thing rough
with the human, on a `spike/<slug>` branch that never merges, cutting every
corner that does not change what they see. Then plan from it or throw it
away. The plan names the branch, says what the spike showed, and says
whether the items bring its code onto their branches and rework it or write
fresh with it as reference. A mission that changes code already understood
needs no spike.

For non-trivial missions, follow **Explore → Decide → Check (DACI)** before
locking the plan:

1. **Explore:** Open `decisions.md` in `.atelier/missions/<id>/` with the 2–5
   real product or architectural forks. Invite cross-lab Contributors with
   disjoint lenses (for example, product/UX vs. architecture/failure modes) to
   read the codebase and append options, tradeoffs, and risks directly to
   `decisions.md` ("yes, and" expansion rather than adversarial tearing down).
2. **Decide:** Collapse each fork to one decision and record the rationale in
   `decisions.md`. The human is the single Approver on product direction, UX,
   and one-way doors; the planner decides reversible engineering forks inside
   the human's Authority.
3. **Check:** Commit the finished plan and its items on the mission's branch
   (`atelier start --mission "<title>"`) and run `atelier submit`. Start two
   independent reviewers, from different labs where two are available, each with
   the task "Run atelier review <mission> and follow what it prints.", in a
   checkout of its own at the pushed commit (`git worktree add --detach`), and
   tell neither about the other. Each publishes its own verdict to the branch.
   Fix what they find, submit again for a new round, and run `atelier merge`
   once both reviewers of a round approve. The merge is the agreement; nobody
   says go after it.

## The Mission Packet

A small mission needs only `mission.md`. A larger or higher-risk mission splits
into up to four focused files in `.atelier/missions/<id>/` with zero
duplication — each fact lives in one place:

- `mission.md` (Product & Outcome — frozen human contract): Why, Outcome,
  **Definition of Done** in `## Success criteria` (both measurable numbers —
  benchmarks, test/coverage/complexity/latency targets — and the subjective
  artifact or walkthrough the human will judge), Scope separating **Enablers**
  (platform, fixture, or harness fixes needed first) from **Delivery** items,
  and Authority.
- `decisions.md` (optional, living — DACI): The 2–5 real forks, options,
  tradeoffs, and decisions with rationale.
- `design.md` (optional — frozen human contract): UX flows, layouts, and
  user-facing copy where applicable.
- `architecture.md` (optional, living): Codebase-verified module/type/API
  contracts, seam ownership, Item 1 as a Walking Skeleton (end-to-end thin
  slice proving the wiring early), known rabbit holes, and shared hotspot files
  or migrations.

`mission.md` and `design.md` are the human's contract (amending them after merge
requires plan review). `architecture.md`, `decisions.md`, and `notes.md` are
living documents that the lead and agents update on backlog branches as code
lands without triggering plan review.

## The Level of a Plan

Write `mission.md` as outcomes: what will be true when the mission closes, how
each criterion is measured or checked, the items in their order (Enablers
first, then Walking Skeleton, then Delivery), what is out, and what the human
withholds. Keep mechanics out of `mission.md` (put verified technical contracts
in `architecture.md` and let each item own its implementation); a plan that
prescribes step-by-step procedures draws findings about the procedure.

Give the items that start first Done when lines that can be checked. Give a
later item, one that builds on work not yet landed, its Goal and what it
owes the Outcome, and do not write the checklist of how it is built: what
lands first changes that, and the lead writes it then
([roles/lead.md](lead.md)).

## Plan Review Rounds

From round 2 on, reviewers check what changed and the findings carried, so
commit each fix and keep other rewriting out of it.

When two rounds in a row of one plan change split, one reviewer approving and
the other asking for changes, stop: `submit` refuses another round. The same
holds for an amendment, whose count starts at none. Bring the open findings
to the human to rule, and write the ruling in the mission's `notes.md`. Then
do as ruled: `ATELIER_SKIP="<ruling>" atelier merge`; or amend, commit, and
`ATELIER_SKIP="<ruling>" atelier submit` for another round; or drop the
change.
