# Work View Ordering

Work views are human queues, not schedulers. They should make dependency shape
visible without inventing work, hiding rows, or replacing explicit blocker
records.

This contract applies to issue-like row sets in:

- `atelier issue list`
- `atelier issue show` subissues
- `atelier issue status <objective-id>`
- explicit objective browsing or inventory views
- root `atelier status`

## Blocker Order

When a command has already selected a visible set of rows, open blocker edges
inside that visible set affect display order:

- if issue B is an open blocker for issue A, B appears before A;
- if a chain or diamond of visible open blockers exists, every visible blocker
  is emitted before the work it blocks;
- blockers outside the visible set are not inserted as phantom rows;
- closed or done blockers do not constrain ordering;
- hidden blockers still affect the blocked row state and compact blocker cue;
- cycles or invalid dependency projections preserve every row and fall back to a
  deterministic display order for the cyclic remainder.

The shared order is layered after the command has chosen its scope. Parent and
child hierarchy, mission membership, filters, search matches, and command
limits decide which rows are visible. The ordering helper then sorts only those
rows.

## Tie Breakers

Rows not constrained by visible open blockers use deterministic tie breakers:

1. row state rank: ready, active, blocked, done, not-ready;
2. priority rank: critical, high, medium, low, then unknown priorities;
3. newest updated timestamp first;
4. issue ID.

Hierarchy may still group rows under a parent heading, but local hierarchy
presentation should not redefine blocker semantics. If a command needs a
different grouping, it should group first and apply the shared order inside the
visible group.

## Row State Vocabulary

Work views use one readable row state label:

- `ready`: todo-category work with no open blockers in the row state context;
- `blocked`: work with one or more open blockers, including hidden blockers
  outside the visible set;
- `active`: active-category work, including configured workflow statuses such
  as `in_progress`, `review`, or `validation`;
- `done`: done-category work;
- `not-ready`: work that is not startable and does not fit a more specific
  work-view state.

Human queue rows should not print duplicate category/status tokens such as
`todo/todo` or `active/in_progress`. Exact configured workflow status is the
primary lifecycle label; categories belong in rollups, filters, and explicit
metadata. Blocked rows should use the `blocked` state plus a compact count or
drill-down cue by default, while detail surfaces preserve exact blocker IDs for
repair.
