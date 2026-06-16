# Command Audit

This audit classifies the current `atelier` CLI surface by the operator role
most likely to reach for each command, then records whether the command is named,
documented, and shaped for that role.

The audit is organized by root command surface. Subcommands are classified inside
the root command file when the root command serves more than one role.

## Category Contract

The product surface uses four command categories:

- Normal workflow: visible operator commands for orientation, work lifecycle,
  proof, closeout readiness, and ordinary health.
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
- Reviewer: reviews code, checks proof, and performs validation.
- Manager/orchestrator: creates and coordinates work, missions, plans,
  subagents, branches, and workspaces.
- Admin: configures, repairs, migrates, and performs explicit maintenance on
  Atelier itself.

## Command Files

- [abandon](abandon.md)
- [branch](branch.md)
- [category review](category-review.md)
- [diagnostics](diagnostics.md)
- [doctor](doctor.md)
- [evidence](evidence.md)
- [export](export.md)
- [graph](graph.md)
- [history](history.md)
- [import-beads](import-beads.md)
- [init](init.md)
- [issue](issue.md)
- [lint](lint.md)
- [maintenance](maintenance.md)
- [man](man.md)
- [mission](mission.md)
- [note](note.md)
- [plan](plan.md)
- [rebuild](rebuild.md)
- [repair](repair.md)
- [role guides](role-guides.md)
- [search](search.md)
- [start](start.md)
- [status](status.md)
- [workflow](workflow.md)
- [worktree](worktree.md)

## Summary

The command surface should stay product-oriented rather than role-prefixed.
`issue create` and `issue transition` belong together because they both mutate
or inspect issue workflow state. Role-specific guidance belongs in a guide
layer: `atelier man worker`, `atelier man reviewer`, `atelier man manager`, and
`atelier man admin`.

The main product risk is not that all roles share commands. The risk is that
general help presents too many commands without answering which role should care.
Role-specific guide pages should present a smaller command path, while root help
continues to expose the product nouns. Current-work orientation should come from
canonical `in_progress` issue records rendered by `status`, `mission status`,
and issue workflow surfaces, not from hidden claim or runtime active-pointer
helpers.
