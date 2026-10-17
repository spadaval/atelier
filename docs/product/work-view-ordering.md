# Work View Ordering

Work views are human queues, not schedulers. They should make dependency shape
visible without inventing work, hiding rows, or replacing explicit blocker
records.

This contract applies to issue-like row sets in:

- `atelier work ready` and `atelier work blocked`
- `atelier work mission <mission-id>` and `atelier work parent <parent-id>`
- `atelier issue show` children and `--full` subissues
- `atelier issue show --full <objective-id>` rollups
- explicit objective browsing views
- root `atelier status`

It does not apply to the neutral ID ordering of `atelier issue list` or the
mission-record ordering of `atelier work missions`; those contracts live in
[Issue Inventory And Mission Overview](issue-inventory-and-mission-overview.md).

## Blocker Order

When a command has already selected a visible set of rows, open blocker edges
inside that visible set affect display order:

- if issue B is an open blocker for issue A, B appears before A;
- if a chain or diamond of visible open blockers exists, every visible blocker
  is emitted before the work it blocks;
- blockers outside the visible set are not inserted as phantom rows;
- closed or done blockers do not constrain ordering;
- hidden blockers still affect the blocked row state and compact blocker cue;
- cycles or invalid dependency graphs preserve every row and fall back to a
  deterministic display order for the cyclic remainder.

The shared order is layered after the command has chosen its scope. Parent and
child hierarchy, mission membership, filters, search matches, and command
limits decide which rows are visible. The ordering helper then sorts only those
rows.

## Tie Breakers

Rows not constrained by visible open blockers use deterministic tie breakers:

1. row state rank: ready, needs human and context; then active, blocked,
   done, backlog;
2. priority rank: critical, high, medium, low, then unknown priorities;
3. newest updated timestamp first;
4. issue ID.

Hierarchy may still group rows under a parent heading, but local hierarchy
presentation should not redefine blocker semantics. If a command needs a
different grouping, it should group first and apply the shared order inside the
visible group.

## Row State Vocabulary

Work views use one readable row state label:

- `ready`: work whose exact configured status exposes a direct transition into
  an active-category status and has no open blockers in the row state context;
- `needs human`: ready work labelled `needs-human`, and unblocked open
  decisions, which only the human can do;
- `blocked`: work with one or more open blockers, including hidden blockers
  outside the visible set;
- `active`: active-category work, including configured workflow statuses such
  as `in_progress`, `review`, or `validation`;
- `done`: done-category work;
- `context`: missions, and parents with open children, which are worked
  through their children;
- `backlog`: work that is not startable and does not fit a more specific
  work-view state. Draft missions and `backlog` work are the planning
  examples: their status shares the `todo` category, but their next transition
  refines or plans them rather than entering active execution. Children of a
  backlog parent are parked with it.

A work view's sections are named by row state (Ready Work, Blocked Work, Done
Work), so their rows carry the exact configured workflow status and not the
state: a todo item under Blocked Work reads `atelier-x ⚠ todo medium Title |
1 blocker` (see the item line in [Human CLI Output](human-cli-output.md);
atelier-tf9n replaces the count with the direct blockers). A list that mixes
states, such as `work mission --all`, leads each row with its state unless
the status word already says it, as in `needs human atelier-x todo medium
Title` or `backlog atelier-y draft high Title`; a backlog-status row needs no
lead. Rows should not print duplicate
category/status tokens such as `todo/todo` or `active/in_progress`; categories
belong in rollups, filters, and explicit metadata. Blocked rows carry a compact
blocker cue, today a count, while detail surfaces preserve exact blocker IDs
for repair.
