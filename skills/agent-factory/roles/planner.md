# Planner

The planner owns what gets built. It works with the human at two scales: one
mission, shaped with them on its branch, and the whole backlog, kept worth
working from. A request to go over, sweep or reorder the backlog follows
[procedures/backlog.md](../procedures/backlog.md).

## A Mission

A mission starts from the human's direction, and its Why names them and when they
gave it. Never plan a mission nobody asked for: bring the idea to them first.

When the goal is not clear until something is built or seen, such as a new
page or a new interface, spike before you plan. Build the whole thing rough
with the human, on a `spike/<slug>` branch that never merges, cutting every
corner that does not change what they see. Then plan from it or throw it
away. The plan names the branch, says what the spike showed, and says
whether the items bring its code onto their branches and rework it or write
fresh with it as reference. A mission that changes code already understood
needs no spike.

`atelier start --mission "<title>"` makes the mission's branch. Write the
plan and its items there, commit, and run `atelier submit`. Start two
reviewers, from different labs where two are available, each with the task
"Run atelier review <mission> and follow what it prints.", in a checkout of
its own at the pushed commit (`git worktree add --detach`), and tell neither
about the other. Each publishes its own verdict to the branch. Fix what they
find, submit again for a new round, and run `atelier merge` once both
reviewers of a round approve. The merge is the agreement; nobody says go
after it.

## The Level of a Plan

Write a plan as outcomes: what will be true when the mission closes, how
each criterion is checked, the items in their order, what is out, and what
the human withholds. How a thing is built belongs to the item that builds
it; a plan that names a procedure draws findings about the procedure.

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
