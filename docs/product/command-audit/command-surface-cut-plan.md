# Command Surface Cut Plan

This checkpoint records the planned simplification of Atelier's CLI surface.
The standard is the Zen principle: every feature must justify its cost, and old
paths should be deleted once the new path is clear.

The only valid reason to keep a command surface is that its job cannot
reasonably be done by another command. Existing implementation, historical
usage, role labels, or compatibility are not enough.

## Target Shape

The public workflow surface should collapse toward a small set of powerful
commands:

- `atelier init`
- `atelier man`
- `atelier status`
- `atelier work ...`
- `atelier issue ...`
- `atelier evidence ...`
- `atelier review ...`
- `atelier history ...`
- `atelier bundle ...`
- one health command, preferably `atelier check`
- `atelier prune`

Everything else is either a filter, an option, an implementation detail, an
admin escape hatch, or obsolete vocabulary.

## Contract Non-Goals

This cut does not introduce a public selector DSL, dashboard language, global
JSON result contract, or replacement search surface. `work` is an operational
view owner for human work queues, not a general query engine. `issue show`
owns bounded objective rollup and record detail, not full recursive bulk
selection. `issue transition` owns lifecycle readiness and mutation, not hidden
status repair. Canonical Markdown/YAML records remain the machine-readable API
unless a deliberately versioned artifact is designed later.

Removed commands must have replacement capability, not replacement spellings:

| Removed surface | Replacement owner |
| --- | --- |
| `mission list` | `issue list --type mission` |
| `mission status <id>` | objective rollup in `issue show <mission-id>` and terminal gates in `issue transition <mission-id>` |
| `issue status <objective-id>` | objective rollup in `issue show <objective-id>` and terminal gates in `issue transition <objective-id>` |
| `issue blocked [<id>]` | blocked inventory in `issue list --blocked`; blocker detail in `issue show <id>` |
| `issue table` | `issue list` filters and list formatting |
| `issue block` / `issue unblock` | `issue link` / `issue unlink --role blocked_by` |
| root `search` | no replacement in this cut; a future search design must justify a cross-record search job |
| scoped `history --issue/--mission/--epic` variants | bounded recent activity in `issue show` where useful; high-level timeline in `history` |
| provider roots such as `forgejo` | review/admin ownership, normally hidden from workflow help |
| visible branch repair | workflow transitions, with hidden/admin recovery only when needed |
| `lint` / `doctor` / `workflow check` / `rebuild` as separate normal surfaces | one `check` health and repair surface |

## Command Jobs

### `atelier status`

Root checkout orientation only.

It answers:

- what is active in this checkout;
- whether the tracker/runtime state is healthy enough to proceed;
- what the next one or two useful commands are.

It must not become an issue detail view, objective report, branch report,
evidence report, or diagnostics console.

### `atelier issue list`

Global issue selection and inventory.

It answers:

- what work exists;
- what is ready;
- what is blocked;
- what matches simple filters such as workflow status, issue type, priority, or
  label.

It should not grow into a scoped query language. Do not reinvent JQL.

Allowed direction:

- `issue list`
- `issue list --ready`
- `issue list --blocked`
- `issue list --status <status>`
- `issue list --type <issue-type>`
- `issue list --label <label>`
- `issue list --priority <priority>`

Avoid unless a later decision explicitly reverses this:

- `--within`
- `--under`
- recursive scope expressions
- arbitrary relationship queries
- a general query DSL

### `atelier issue show <id>`

The durable record detail view.

It answers:

- what this issue is;
- what its workflow status is;
- what text, outcome, evidence expectations, blockers, links, and recent
  activity are recorded;
- what command is most likely next.

If the issue is objective-shaped, including mission or epic records, this view
also owns objective rollup. That rollup is not a separate command.

Objective rollup means:

- work linked to the objective by objective/work relationship roles, such as
  `advances`;
- descendants of linked work roots where hierarchy applies;
- ready/blocked/done/backlog counts;
- direct objective blockers;
- a bounded sample of ready and blocked work;
- terminal readiness summary.

The rollup should be concise and bounded. Full work selection remains global
`issue list`; lifecycle gates remain `issue transition`.

### `atelier issue transition <id> [transition]`

The workflow gate and lifecycle mutation surface.

It answers:

- what transitions are possible;
- why a transition is blocked;
- what validators or branch actions are involved;
- how to execute the transition.

No separate `--options` command should be necessary long term. If no transition
name is supplied, the command should show available transitions. Executing a
transition remains explicit.

### `atelier evidence ...`

First-class proof records.

Keep:

- `evidence record`
- `evidence show`
- `evidence list`

Remove or fold:

- `evidence attach`

Attaching existing evidence is a relationship mutation. Prefer a general issue
linking path or evidence recording with an explicit target rather than a
separate attach verb.

### `atelier review ...`

Configured review artifact operations.

Collapse toward:

- `review open`
- `review show`
- `review submit`
- `review resolve`
- `review merge`

Provider plumbing and branch/title/body defaults should be inferred from the
issue wherever possible. Normal users should not have to supply implementation
fields that Atelier already knows.

### Health Command

There should be one normal health/check command.

Preferred target:

- `atelier check`

It should absorb the useful parts of:

- `lint`
- `doctor`
- hidden `workflow check`
- hidden projection/runtime diagnostics when needed for public recovery

If renaming is too disruptive in the first implementation slice, `lint` may
temporarily remain the spelling, but the product target is one health command,
not separate lint/doctor/workflow/rebuild/export surfaces.

## Remove Or Fold

### Remove `atelier mission`

Mission is an issue type, not a command namespace.

Replace:

- `mission list` with `issue list --type mission`
- `mission status <id>` with `issue show <mission-id>`

Mission-specific status behavior should become generic objective rollup inside
`issue show`.

### Remove `atelier issue status`

The workflow status field is valuable. The `issue status` command is not.

The current command name is misleading because it does not simply show an
issue's workflow status. It performs an objective health rollup. That rollup
belongs in `issue show <objective-id>` for objective-shaped issues.

Replace:

- `issue status <id>` with `issue show <id>`
- terminal readiness detail with `issue transition <id>`

Do not add `issue progress` unless future use proves that objective rollup in
`issue show` is insufficient.

### Remove `atelier issue blocked`

Blocked work is not a command noun.

Replace:

- blocked-work inventory with `issue list --blocked`
- blocker detail with `issue show <id>`

Do not keep a separate blocker drilldown command just because it already exists.

### Remove `atelier issue table`

Tables are output format, not a command.

Replace:

- `issue table --kind mission` with `issue list --type mission`
- `issue table --kind issue` with `issue list`

If table output is needed, add a format flag or allow `issue list` to choose a
table layout when appropriate.

### Remove `issue block` And `issue unblock`

Blocking is a relationship.

Replace with the general relationship surface:

- `issue link <blocked-id> <blocker-id> --role blocked_by`
- `issue unlink <blocked-id> <blocker-id> --role blocked_by`

If that spelling is too awkward, improve `issue link`, not by adding separate
block/unblock verbs.

### Remove `search`

Search is not a command Atelier should preserve during this simplification.
Keeping it invites another broad, underspecified discovery surface, and folding
it into `issue list --query` would recreate the same ambiguity under a quieter
name.

Remove:

- `search`
- root help and docs that teach search as a normal workflow

Do not add `issue list --query` as a compatibility replacement. Add a search
surface later only if a new design proves a stronger cross-record search job.

### Keep High-Level `history`

Atelier still needs one high-level history view.

Keep:

- high-level repository or tracker timeline browsing;
- bounded, readable output for recent activity across the project;
- enough filtering to make the high-level timeline usable without becoming a
  scoped query language.

Remove or fold:

- issue-, mission-, and epic-specific history variants that duplicate
  `issue show` detail;
- descendant-scoped history modes unless a focused command proves necessary;
- raw timeline dumps in default issue detail output.

More specific history should move into the command that owns that record or be
removed.

### Keep `man` As Smart Guidance

Role guidance is valuable as a slightly smart guide, not as another workflow
surface. It should route users toward the surviving commands and adapt to the
repository's current tracker state where that helps.

Keep:

- `man`
- role-oriented guidance such as worker, reviewer, validator, manager, and
  admin routes;
- links from `man` into current command help and status next actions.

Remove:

- guidance for removed commands;
- duplicate procedural flows that conflict with the CLI's real next actions.

### Keep `bundle` For Now

Bundle is not ideal as a root noun, but the replacement is not yet better.
`issue apply` is also awkward because bulk graph preview/apply is broader than
ordinary single-issue mutation.

Keep:

- `bundle preview <file>`
- `bundle apply <file>`

The command remains a bounded owner for graph preview/apply until a clearly
better product owner is proven. Do not churn this surface just to make the root
list smaller.

### Fold `forgejo`

Forgejo is provider setup for reviews.

Replace:

- `forgejo roles check` with `review provider check`
- `forgejo roles provision` with `review provider provision`

Provider-specific admin commands should not be part of the normal workflow
surface.

### Hide Or Remove `branch`

Normal branch behavior belongs to workflow transitions.

Replace:

- `branch for-epic` with `issue transition <id> start`
- `branch status` with `status` when branch state matters
- `branch merge` with `issue transition <id> close` or another workflow
  transition that owns integration

Manual branch repair can exist only as a hidden/admin recovery path if workflow
transitions cannot reasonably do the job.

### Hide Or Remove `maintenance`

Public destructive record surgery is not normal workflow.

Keep only if there is a clear admin-only escape hatch, and keep it out of root
workflow guidance. Otherwise prefer `prune` for supported cleanup and Git for
recovery.

### Merge `lint`, `doctor`, `workflow`, `export`, And `rebuild`

There should be one health/recovery surface.

Replace:

- `lint` with `check`
- `doctor` with `check --runtime` or `check --fix`
- hidden `workflow check` with `check --workflow` if raw workflow diagnostics
  are still needed
- `rebuild` with `check --fix`
- `export --check` with internal validation or `check --determinism` only if a
  user-facing need remains

Hidden implementation probes should not appear as normal commands.

### Remove Hidden Migration Surfaces When Possible

Remove or keep strictly temporary:

- `import-beads`
- `export`
- `rebuild`

Normal setup should use `init`, with `init --import-beads` only if the migration
path is still needed.

## Review Surface Collapse

Current review commands are too many.

Target:

- `review open [--issue <id>] [--existing <url-or-number>]`
- `review show [--issue <id>] [--comments] [--unresolved]`
- `review submit [--issue <id>] --approve|--request-changes|--comment "..."`
- `review resolve [--issue <id>] <finding>`
- `review merge [--issue <id>]`

Remove or fold:

- `review link`
- `review status`
- `review comments`
- `review comment`
- `review approve`
- `review request-changes`

## Migration Principle

Do not add compatibility aliases.

When a replacement is implemented and clear:

- delete the old command path;
- update help and docs;
- update tests to assert rejection of removed commands only where that improves
  product clarity;
- keep next-command output pointed at the surviving surface.

## First Implementation Cuts

Recommended first slice:

1. Remove visible `mission`; move mission health into objective rollup in
   `issue show`.
2. Remove `issue status`; put objective rollup in `issue show`.
3. Remove `issue blocked`; rely on `issue list --blocked` and `issue show`.
4. Remove `issue table`; use `issue list --type mission`.
5. Remove `issue block` and `issue unblock`; use `issue link/unlink --role
   blocked_by`.
6. Remove root `search` entirely, including help/docs references.
7. Keep high-level `history`, but remove or fold scoped history variants.
8. Keep `bundle` as the graph preview/apply owner for now.
9. Keep `man` as smart guidance and remove stale guidance for deleted commands.
10. Start merging `lint` and `doctor` into one `check` surface.

These cuts attack the worst proliferation without creating a new query
language.
