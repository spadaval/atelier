# Command Audit

This audit classifies the current `atelier` CLI surface by the operator role
most likely to reach for each command, then records whether the command is named,
documented, and shaped for that role.

The audit is organized by root command surface. Subcommands are classified inside
the root command file when the root command serves more than one role.

Last refreshed: 2026-06-29 from `target/debug/atelier --help`, focused
subcommand help, command-audit consistency checks, and live dashboard/transition
samples.

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

Every command also has a [complexity budget](complexity-budget.md). Commands do
not survive because they already exist, mirror provider verbs, expose internal
machinery, or make an implementation easier. If a command is expensive to
explain and low-value to operate, fold it, hide it, or remove it.

## Roles

- Worker: implements assigned work and leaves durable handoff context.
- Reviewer: reviews code and checks proof.
- Validator: runs explicit validation work and records validation proof.
- Manager/orchestrator: creates and coordinates work, missions, ordinary
  planning artifacts, subagents, branches, and workspaces.
- Admin: configures, repairs, migrates, and performs explicit maintenance on
  Atelier itself.

## Current Help-Visible Root Command Files

- [bundle](bundle.md)
- [check](check.md)
- [evidence](evidence.md)
- [history](history.md)
- [init](init.md)
- [issue](issue.md)
- [man](man.md)
- [prune](prune.md)
- [review](review.md)
- [status](status.md)
- [work](work.md)

## Hidden Advanced Or Migration Command Notes

These files document callable commands that are hidden from root help and must
not be taught as ordinary workflow:

- [diagnostics](diagnostics.md): hidden local command telemetry.
- [export](export.md): hidden deterministic-renderer diagnostic or migration
  helper.
- [branch](branch.md): hidden/manual owner-branch recovery; routine branch
  guidance comes from status, work dashboards, issue detail, and transitions.
- [doctor](doctor.md): hidden legacy repair entry; normal repair starts from
  visible `check` and explicit `check --fix`.
- [forgejo](forgejo.md): hidden provider setup diagnostics.
- [import-beads](import-beads.md): hidden explicit predecessor import escape
  hatch; normal setup uses `init --import-beads`.
- [lint](lint.md): hidden compatibility health probe; normal validation uses
  visible `check`.
- [maintenance](maintenance.md): hidden danger-zone maintenance primitives.
- [rebuild](rebuild.md): hidden projection diagnostic; operator repair starts
  from `check --fix`.
- [workflow](workflow.md): hidden raw workflow-policy diagnostics.

## Cross-Cutting Audit Artifacts

- [actual agent complaints](agent-complaints.md)
- [category review](category-review.md)
- [command complexity budget](complexity-budget.md)
- [export check reference classification](export-check-reference-classification.md)
- [human output refresh](human-output-refresh.md)
- [role guides](role-guides.md)

## Retired Or Deferred Notes

These files record surfaces that should not be treated as current root command
documentation:

- [abandon](abandon.md): removed legacy active-pointer cleanup.
- [graph](graph.md): removed abstract relationship namespace; use issue detail,
  work dashboards, and blocker views.
- [mission](mission.md): removed parallel objective namespace; use mission-typed
  issue records.
- [note](note.md): removed generic note surface; use `issue note`.
- [plan](plan.md): deferred plan CRUD; current one-shot batch creation is
  `bundle`.
- [repair](repair.md): removed root repair surface; use `check --fix` for
  ignored local runtime/projection repair and normal issue transitions for
  durable workflow state.
- [search](search.md): removed root search surface pending a stronger
  cross-record search design.
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
come from canonical in-progress issue records rendered by `status`, type-aware
`issue show <objective-id>`, and issue workflow surfaces, not from separate
runtime active-pointer helpers.

## Cutting Findings

| Finding | Evidence | Next step |
| --- | --- | --- |
| The visible root surface is now the reduced operator set. | Root help lists `init`, `man`, `status`, `work`, `issue`, `bundle`, `evidence`, `review`, `history`, `check`, and `prune`. | Keep current command files limited to help-visible roots; classify hidden and retired files separately. |
| Hidden advanced commands remain callable but are not root-help surfaces. | `workflow`, `diagnostics`, `export`, `rebuild`, and `import-beads` respond to focused help, while root help omits them. | Keep them in hidden advanced or migration notes; do not cite them as routine worker/reviewer paths. |
| Retired command notes were still mixed with current command files. | `mission`, `graph`, `plan`, `start`, `worktree`, `repair`, `note`, and `abandon` are rejected as unrecognized root commands. | Leave them only as retired/deferred audit notes or delete them after remaining references are gone. |
| `graph` was an abstract helper namespace compensating for underpowered record views. | `graph` is now rejected; prior `graph impact` and `graph tree` behavior belongs in issue detail/status and blocker views. | Removed. Use `issue show` for downstream impact and `issue show <objective-id>` for objective hierarchy and work health. |
| The audit previously overloaded `work queue`. | Older docs routed mission inventory and generic issue inventory through `work queue --type mission`, but the current direction restores `issue list` as a simple inventory command and keeps mission dashboards under `work mission`. | Update docs and help so inventory, operational queues, and mission dashboards are separate jobs. |
| Root help claims issue listing before the subcommand exists. | `target/debug/atelier --help` says `issue` can "Create, list, show..." but `target/debug/atelier issue --help` has no `list` subcommand. | Track this as current drift until `atelier issue list` lands. |
| `mission` was a parallel objective namespace. | `mission` is now rejected; root help teaches mission-typed issue records and `work mission <mission-id>`. | Keep mission guidance under typed issue records and work dashboards rather than aliases. |
| Complex commands need explicit budget verdicts. | `review` mirrors provider verbs, scoped `history` risks query-language sprawl, `evidence attach` duplicates relationship mutation, and transition output dumps implementation machinery. | Apply Keep/Simplify/Fold/Hide/Remove verdicts in each command file before adding new surfaces. |
| `review open` exposes provider plumbing as required operator input. | `atelier review open --help` requires `--title`, `--body`, `--source-branch`, and `--target-branch`; the product contract says lifecycle/status output should route review artifacts. | Refine `review open` toward issue-derived defaults or move the fully manual form to admin/advanced guidance. |
| Human output has recurring scanability debt. | Sampled queue, detail, transition, history, evidence, and role-guide outputs repeat inline commands, overuse `key=value`, print raw activity fields, and lack interactive color. | Use the [human output refresh](human-output-refresh.md) audit to drive the formatter pass before changing command behavior. |
| Actual agents hit trust and guidance failures beyond formatting. | The [actual agent complaint audit](agent-complaints.md) found stale status/projection signals, hidden ready work, parent-blocker ambiguity, duplicate lifecycle paths, implementation-shaped command names, and stale help flags. | Treat the UX refresh as a command-language and trust-state pass, not only a color/layout formatter pass. |
| Retired implementation owners have been removed. | Bundle behavior is owned by `commands::bundle`; stale `label`, `plan`, and `tested` command modules are no longer compiled. | Keep new implementation modules aligned with visible product surfaces. |
| Test infrastructure no longer preserves removed command shapes. | Integration and smoke harnesses execute the arguments supplied by each test directly; old-shape coverage lives in explicit rejection tests. | Keep new tests on the current command surface and avoid ignored compatibility suites. |
| CLI command dispatch is becoming a module-boundary bottleneck. | `crates/atelier-cli/src/main.rs` still owns the root enum and subcommand enums, but issue subcommand dispatch now lives in the current-surface `issue_cli` adapter. | Continue splitting by product surface or use-case boundary before adding new command families. |
