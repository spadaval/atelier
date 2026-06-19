# Command Audit

This audit classifies the current `atelier` CLI surface by the operator role
most likely to reach for each command, then records what that operator is trying
to do, what information they need at that moment, and where the command should
point them next.

The audit is organized by root command surface. Subcommands are classified inside
the root command file when the root command serves more than one role.

Reference principle: [Zen Of Atelier](../zen.md) says default views should
emphasize current, mission-relevant, actionable information, and guidance should
appear at friction points such as blocked, invalid, stale, or missing-proof
state.

## Category Contract

The product surface uses four command categories:

- Normal workflow: visible operator commands for orientation, work lifecycle,
  proof, terminal readiness, and ordinary health.
- Admin maintenance: visible setup, explicit repair, destructive maintenance,
  and manual owner-branch/worktree recovery.
- Hidden debug diagnostics: raw policy, telemetry, projection, or
  deterministic-renderer probes that are callable only for targeted diagnostics,
  tests, or migration work.
- Temporary migration: transitional inherited-state or deterministic-rendering
  helpers that must not become compatibility promises or normal workflow gates.

See [category review](category-review.md) for examples, excluded non-examples,
and boundary decisions.

## Roles

- Worker: implements assigned work and leaves durable handoff context.
- Reviewer: reviews code and checks proof.
- Validator: runs explicit validation work and records validation proof.
- Manager/orchestrator: creates and coordinates work, missions, ordinary
  planning artifacts, subagents, branches, and workspaces.
- Admin: configures, repairs, migrates, and performs explicit maintenance on
  Atelier itself.

## Command Files

### Active Public Surface

- [branch](branch.md)
- [doctor](doctor.md)
- [evidence](evidence.md)
- [forgejo](forgejo.md)
- [graph](graph.md)
- [history](history.md)
- [init](init.md)
- [issue](issue.md)
- [lint](lint.md)
- [maintenance](maintenance.md)
- [man](man.md)
- [mission](mission.md)
- [review](review.md)
- [role guides](role-guides.md)
- [search](search.md)
- [session](session.md)
- [start](start.md)
- [status](status.md)
- [worktree](worktree.md)

### Hidden Diagnostic And Migration Surface

- [diagnostics](diagnostics.md)
- [export](export.md)
- [import-beads](import-beads.md)
- [rebuild](rebuild.md)
- [workflow](workflow.md)

### Retired Or Deferred Surface

- [abandon](abandon.md)
- [note](note.md)
- [plan](plan.md)
- [repair](repair.md)

### Supporting Reviews

- [category review](category-review.md)
- [export check reference classification](export-check-reference-classification.md)

## Summary

The command surface should stay product-oriented rather than role-prefixed.
`issue create` and `issue transition` belong together because they both mutate
or inspect issue workflow state. Role-specific guidance belongs in a guide
layer: `atelier man worker`, `atelier man reviewer`, `atelier man validator`,
`atelier man manager`, and `atelier man admin`.

The main product risk is not that all roles share commands. The risk is that
general help presents too many commands without answering which role should care.
Role-specific guide pages should present a smaller command path, while root help
continues to expose the product nouns. Current-work orientation should come from
canonical `in_progress` issue records rendered by `status`, `mission status`,
and issue workflow surfaces, not from hidden claim or runtime active-pointer
helpers.

## Current Surface Snapshot

Visible root help currently exposes:

| Category | Commands |
| --- | --- |
| Setup | `init` |
| Orientation | `man`, `status`, `start` |
| Issues | `issue`, `search`, `graph` |
| Missions and planning | `mission`, `bundle` |
| Records | `evidence`, `session`, `review`, `forgejo`, `history` |
| Advanced work | `worktree`, `branch` |
| Maintenance | `maintenance`, `lint`, `doctor` |

Hidden source-defined commands are `export`, `rebuild`, `import-beads`,
`workflow`, and `diagnostics`. They may support tests, migration, projection
repair, or targeted debugging, but they are not normal worker, reviewer, or
manager workflow.

## Cross-Surface Findings

- Role guides are the right orientation layer, but in this checkout
  `atelier man <role>` fails before rendering because `.atelier/workflow.yaml`
  contains a local `effects` field that the installed binary rejects. That is a
  strong friction-point failure: the recovery text names `atelier man admin`,
  but the admin guide is reached through the same failing stateful load.
- The active surface has moved beyond the older audit list. `bundle`,
  `session`, `review`, and `forgejo` need first-class audit pages because they
  now appear in root help.
- The audit should keep hidden and retired pages, but root summaries must not
  imply those commands are active public workflow.
- Several commands have accurate help text but weak orientation about role and
  next action. The highest-impact gaps are `review`, `session`, `bundle`,
  `forgejo`, `worktree`, and `branch`, because each is easy to confuse with
  workflow transitions, evidence, or routine worker flow.
