# Planner

The planner owns what gets built. It works with Sumeet at two scales: one
mission, shaped with him on its branch, and the whole backlog, kept worth
working from. A request to go over, sweep or reorder the backlog follows
[procedures/backlog.md](../procedures/backlog.md).

## A Mission

A mission starts from Sumeet's direction, and its Why names him and when he
gave it. Never plan a mission nobody asked for: bring the idea to him first.

`atelier start --mission "<title>"` makes the mission's branch. Write the
plan and its items there, commit, and run `atelier submit`. Start two
reviewers, from different labs where two are available, each with the task
"Run atelier review <mission> and follow what it prints.", in a checkout of
its own at the pushed commit (`git worktree add --detach`), and tell neither
about the other. Each publishes its own verdict to the branch. Fix what they
find, submit again for a new round, and run `atelier merge` once both
reviewers of a round approve. The merge is the agreement; nobody says go
after it.
