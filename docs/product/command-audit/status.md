# `atelier status`

Primary role: Worker.

Primary question: "What is in progress, ready, stale, or unsafe in this
checkout?"

## Assessment

- Name: Correct. `status` is the expected root orientation command.
- Documentation: Correct. It should be the first role-specific command for
  workers and a common sanity check for all other roles.
- Design: Correct if it derives current work from checked-out issue statuses and
  points to scoped drill-downs instead of becoming mission closeout, issue
  detail, or health detail itself.
- Output hierarchy: Current `in_progress` issue set, active mission, ready
  count, freshness, then one or two next commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier status` | Worker | Choose the next local action. | Good. |
