# Retired `atelier mission`

Primary role: Retired namespace; current objective coordination uses
mission-typed issue records plus `work mission` dashboards.

Primary question: "How do I create, focus, inspect, coordinate, and close a
durable mission?"

## Current Contract

Missions are no longer a parallel command namespace. A mission is an issue
record whose `type` is `mission`, with typed sections for intent, constraints, risks,
validation, linked work, mission blockers, and closeout notes. Operators use
the general issue surfaces to create, inspect, link, transition, note, and close
that record.

The replacement command model is:

- `atelier issue create --issue-type mission` creates mission-shaped
  coordination records through the general issue creation surface. Existing
  mission records are migrated directly instead of being wrapped by
  compatibility aliases.
- `atelier issue show <objective-id>` is the rich objective detail view. It
  owns intent, constraints, risks, validation criteria, linked work, blockers,
  hierarchy, evidence, and affected-record context.
- `atelier work mission <objective-id>` is the mission orchestration dashboard.
  It owns mission-scoped ready, active, blocked, and done work views.
- `atelier issue transition <objective-id>` owns terminal readiness gates.
- `atelier issue link <objective-id> <issue-id> --role advances` and
  `atelier issue unlink ...` replace mission work-link commands. Relationship
  roles are part of the general issue link contract, not a generic root `link`
  namespace.
- `atelier issue link <objective-id> <blocker-id> --role blocked_by` records
  objective blockers through the same typed issue-link surface as ordinary work
  blockers.
- `atelier issue note <objective-id> "..."` records coordination notes and
  closeout reasons. Closing an objective uses the normal workflow transition
  path, so close reasons are transition notes rather than a mission-specific
  `close --reason` command.
- `atelier issue transition <objective-id> close --reason "..."` closes the
  objective only after the type-aware status gates pass.

`mission start` and active mission focus are removed rather than renamed.
Checkout orientation comes from root `atelier status`, which renders the
current in-progress issue set and can show objective context when the checkout
is carrying objective work. Static mission focus state should not be kept as a
second workflow pointer.

Removed lifecycle and mutation forms should fail as unknown commands or invalid
subcommands without compatibility guidance. There is no staged deprecation, no
`mission` alias to issue mutation, and no fallback for old mission-only
mutation shapes unless a human explicitly asks for a compatibility window.

Current verification: `target/debug/atelier mission --help` is rejected because
the root namespace is retired.

## Assessment

- Name: Removed. Mission is a durable purpose boundary, not a root command.
- Documentation: Operator guidance should teach mission-typed issue records,
  `work mission`, `issue show`, and `issue transition`.
- Design: Consolidate both reads and mutation. Mission relationship mutations
  move into typed issue links; orchestration reads move to `work mission`.
- Output hierarchy: Objective identity and lifecycle first, current
  work/blockers next, linked hierarchy and affected records next,
  proof/health/terminal readiness next, then specific next actions.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `mission create` | Manager/orchestrator | Create mission purpose, constraints, risks, validation criteria. | Removed. Replacement: `issue create --issue-type mission`. |
| `mission show` | Manager/orchestrator | Inspect rich mission prose and relationship context. | Removed. Replacement: `issue show <objective-id>`. |
| `mission start --switch` | Manager/orchestrator | Set active mission focus. | Removed. Root status and canonical in-progress issue records own checkout orientation. |
| `mission status` | Manager/orchestrator | See current mission health and next actions. | Removed. Replacement: `work mission <id>` and `issue transition <id>`. |
| `mission status --verbose` | Reviewer | Inspect terminal-check detail. | Removed. Replacement: `issue transition <id> --verbose` once verbose transition output lands. |
| `mission close --reason` | Manager/orchestrator | Close a mission after gates pass. | Removed. Replacement: `issue transition <objective-id> close --reason`. |
| `mission list` | Manager/orchestrator | Select current missions and health summaries. | Removed. Replacement: `issue list --issue-type mission` once issue inventory lands. |
| `mission update` | Manager/orchestrator | Change lifecycle fields and mission sections. | Removed. Replacement: `issue update` for fields and Markdown section edits for rich prose. |
| `mission note` | Manager/orchestrator | Add durable coordination or handoff context. | Removed. Replacement: `issue note <objective-id>`. |
| `mission add-work` | Manager/orchestrator | Link issue work into mission scope. | Removed. Replacement: `issue link <objective-id> <issue-id> --role advances`. |
| `mission unlink` | Manager/orchestrator | Remove issue work from mission scope. | Removed. Replacement: `issue unlink <objective-id> <issue-id> --role advances`. |
| `mission add-blocker` | Manager/orchestrator | Mark an issue as a mission blocker. | Removed. Replacement: `issue link <objective-id> <blocker-id> --role blocked_by`. |
