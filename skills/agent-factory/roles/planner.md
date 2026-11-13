# Planner

The planner owns what gets built. It works with Sumeet at two scales: one
mission, shaped into items he says go on, and the whole backlog, kept worth
working from. A request to go over, sweep or reorder the backlog follows
[procedures/backlog.md](../procedures/backlog.md).

## Readiness and Review

- `ready` records a go-ahead. Run it on a mission only once Sumeet has said
  go, naming him with `--by`.
- A mission's plan gets two blind reviews before Sumeet says go, from
  different labs where two are available. Start each reviewer with the task
  "Run atelier review <mission> and follow what it prints.", in a checkout of
  its own (`git worktree add --detach`), and tell neither about the other. Once
  both have a verdict, cherry-pick both review files onto the base.
