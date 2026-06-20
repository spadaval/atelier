# Command Audit

This audit classifies the current `atelier` CLI surface by the operator role
most likely to reach for each command, then records whether the command is named,
documented, and shaped for that role.

The audit is organized by root command surface. Subcommands are classified inside
the root command file when the root command serves more than one role.

Last refreshed: 2026-06-20 from `cargo run -q -p atelier-cli -- --help`.

Current checkout caveat: `atelier status` and `atelier issue status` fail in
this checkout because `.atelier/workflow.yaml` contains
`statuses.in_progress.role`, while the current workflow parser expects
`category`. That makes the command surface available for audit, but the
role/live-state guides cannot be validated from this worktree until the tracked
workflow configuration and binary agree.

## Category Contract

The product surface uses four command categories:

- Normal workflow: visible operator commands for orientation, work lifecycle,
  proof, terminal readiness, and ordinary health.
- Admin maintenance: visible setup, explicit repair, destructive maintenance,
  and manual owner-branch recovery.
- Hidden debug diagnostics: raw policy, telemetry, projection, or
  deterministic-renderer probes that are callable only for targeted diagnostics,
  tests, or migration work.
- Temporary migration: transitional inherited-state or deterministic-rendering
  helpers that must not become compatibility promises or normal workflow gates.

See [category review](category-review.md) for examples, excluded non-examples,
and boundary decisions.

## Consolidation Lens

Per [Zen Of Atelier](../zen.md), command cuts should favor a smaller number of
powerful domain commands over narrow special cases. When a helper command exists
because a core record view cannot answer the operator's question, the preferred
fix is to strengthen the record view. Avoid abstract namespaces that expose the
data model instead of the work object the operator is trying to understand.

## Roles

- Worker: implements assigned work and leaves durable handoff context.
- Reviewer: reviews code and checks proof.
- Validator: runs explicit validation work and records validation proof.
- Manager/orchestrator: creates and coordinates work, missions, ordinary
  planning artifacts, subagents, branches, and workspaces.
- Admin: configures, repairs, migrates, and performs explicit maintenance on
  Atelier itself.

## Current Root Command Files

- [branch](branch.md)
- [bundle](bundle.md)
- [category review](category-review.md)
- [diagnostics](diagnostics.md)
- [doctor](doctor.md)
- [evidence](evidence.md)
- [export](export.md)
- [export check reference classification](export-check-reference-classification.md)
- [forgejo](forgejo.md)
- [graph](graph.md)
- [history](history.md)
- [import-beads](import-beads.md)
- [init](init.md)
- [issue](issue.md)
- [lint](lint.md)
- [maintenance](maintenance.md)
- [man](man.md)
- [mission](mission.md)
- [prune](prune.md)
- [review](review.md)
- [rebuild](rebuild.md)
- [role guides](role-guides.md)
- [search](search.md)
- [status](status.md)
- [workflow](workflow.md)

## Retired Or Deferred Notes

These files record surfaces that should not be treated as current root command
documentation:

- [abandon](abandon.md): removed legacy active-pointer cleanup.
- [note](note.md): removed generic note surface; use `issue note` and
  `mission note`.
- [plan](plan.md): deferred plan CRUD; current one-shot batch creation is
  `bundle`.
- [repair](repair.md): removed root repair surface; use `doctor --fix` for
  ignored local runtime/projection repair and normal issue transitions for
  durable workflow state.
- [worktree](worktree.md): removed visible workspace-management surface pending
  redesign.
- [start](start.md): removed duplicate root lifecycle verb; use
  `issue transition <id> start`.

## Summary

The command surface should stay product-oriented rather than role-prefixed.
`issue create` and `issue transition` belong together because they both mutate
or inspect issue workflow state. Role-specific guidance belongs in a guide
layer: `atelier man worker`, `atelier man reviewer`, `atelier man validator`,
`atelier man manager`, and `atelier man admin`.

The main product risk is not that all roles share commands. The risk is that
general help presents too many commands without answering which role should
care. Role-specific guide pages should present a smaller command path, while
root help continues to expose the product nouns. Current-work orientation should
come from canonical `in_progress` issue records rendered by `status`, `mission
status`, and issue workflow surfaces, not from separate runtime active-pointer
helpers.

## Cutting Findings

| Finding | Evidence | Next step |
| --- | --- | --- |
| Audit docs had fallen behind the visible root surface. | Root help lists `bundle`, `review`, `forgejo`, and `prune`; the audit index did not have command pages for them. | Keep the new pages current with help-visible root commands. |
| Retired command notes were listed as current command files. | `abandon`, `note`, `plan`, and `repair` do not appear in root help, but were peers in the command file list. | Leave them only as retired/deferred audit notes or delete them after any remaining references are gone. |
| `graph` was an abstract helper namespace compensating for underpowered record views. | `graph impact` asked operators to leave `issue` or `mission` to answer blast-radius questions, while `graph tree` overlapped with mission/issue hierarchy views and hard-coded predecessor `todo/done/all` filter language. | Removed. Use `issue show` for downstream impact and `issue status <objective-id>` for objective hierarchy and work health. |
| `review open` exposes provider plumbing as required operator input. | `atelier review open --help` requires `--title`, `--body`, `--source-branch`, and `--target-branch`; the product contract says lifecycle/status output should route review artifacts. | Refine `review open` toward issue-derived defaults or move the fully manual form to admin/advanced guidance. |
| Forgejo review validation is split across layers. | `crates/atelier-app/src/pr.rs` parses and validates linked pull-request state for merge confirmation, while `crates/atelier-cli/src/commands/workflow.rs` separately parses the same review field shape for transition validation. | Move the shared review-link contract into the app layer so CLI transition checks call one canonical helper. |
| Active implementation modules retain removed product names. | `commands::bundle` delegates to `commands::plan`; `commands::tested` is still compiled but is not referenced by current dispatch; `commands::label` has no current dispatch reference. | Delete unused modules and rename bundle implementation ownership after tests prove no remaining internal users. |
| Test infrastructure still preserves removed commands. | Integration/smoke harnesses translate old command shapes such as `issue label`, `issue comment`, `issue next`, `issue tree`, and `issue subissue`; the ignored legacy-surface suite still contains 61 "obsolete legacy command surface removed" tests. | Replace remaining translated call sites with current commands, then delete the translator layer and obsolete ignored tests. |
| CLI command dispatch is becoming a module-boundary bottleneck. | `crates/atelier-cli/src/main.rs` owns the root enum, subcommand enums, helper functions, and dispatch; `commands/workflow.rs` is about 4k lines and `commands/mission.rs` about 2.5k. Normal issue behavior has moved from the obsolete `commands/agent_factory.rs` boundary into `commands/issue.rs`, but the new issue module is still broad. | Continue splitting by product surface or use-case boundary before adding new command families; cutting work should start with removed/hidden surfaces before broad refactors. |
