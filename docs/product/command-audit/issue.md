# `atelier issue`

Primary role: Manager/orchestrator.

Primary question: "How do I create, inspect, mutate, and advance accountable
issue work, including objective records that replace missions?"

`issue` is intentionally shared by roles. The root noun is correct because the
command owns issue records and issue workflow state, not a single user persona.
`issue create` and `issue transition` belong in this family because both are
issue-state operations.

## Objective Records

The target command model treats missions as typed objective issues instead of a
parallel root namespace. Objective records use issue identity, workflow,
blockers, links, notes, evidence, and history. Type-aware issue reads own the
mission-shaped sections that operators need for coordination: intent,
constraints, risks, validation criteria, linked work, objective blockers,
terminal readiness, and closeout notes.

New or changed issue forms needed for that target are:

- `issue create --issue-type objective` for mission-shaped coordination work.
- `issue show <objective-id>` for rich objective detail, hierarchy, blockers,
  evidence, and affected-record context.
- `issue status <objective-id>` for objective health, ready and blocked work,
  proof gaps, terminal readiness, and completion gates.
- `issue link <objective-id> <issue-id> --role advances` and `issue unlink ...`
  for work membership relationships.
- `issue transition <objective-id> close --reason "..."` for objective
  completion, with the close reason stored as transition note activity.

These forms replace `mission create/show/status/close/list/update/note`,
`mission add-work`, `mission unlink`, and `mission add-blocker`. They do not
create compatibility aliases for the removed mission commands.

## Assessment

- Name: Correct. The noun maps to the durable accountability unit.
- Documentation: Needs role-aware examples. Generic `issue --help` is accurate
  but does not tell a worker which subset matters.
- Design: Mostly correct. The family is broad, but cohesion is still issue
  state and issue relationships. This is where issue-sourced relationship and
  blast-radius context now lives after retiring the separate graph namespace.
- Output hierarchy: For reads, current state and blockers before metadata, then
  parent/children, mission links, evidence, relationship context, and affected
  records that may need review. For mutations, changed fields or transition
  result first, then canonical path and next commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `issue create` | Manager/orchestrator | Create actionable issue-shaped work. | Good. Keep `--issue-type` and template behavior explicit. |
| `issue list` | Worker | Find candidate or assigned work. | Good, but role guides should prefer `--ready`, `--blocked`, and exact status/category examples. |
| `issue show` | Worker | Understand the work slice, proof expectations, and relationship context. | Good. It includes issue-scoped downstream impact so operators do not leave the issue view for blast-radius context. |
| `issue transition` | Reviewer | Inspect or execute workflow gates. | Good. It belongs with issue mutation; `--options` should be the reviewer entry point. |
| `issue update` | Manager/orchestrator | Correct issue metadata, parent, labels, type, priority. | Good. Current work is derived from canonical issue status plus checkout context, not from separate runtime ownership state. |
| `issue note` | Worker | Add durable progress, handoff, or observation context. | Good. |
| `issue status` | Worker/reviewer | Inspect type-aware objective health and terminal readiness. | Add. This absorbs the useful `mission status` behavior for objective records. |
| `issue link` / `issue unlink` | Manager/orchestrator | Add or remove typed issue relationships such as objective work membership. | Add. Keep relationship mutation under issue records rather than a generic root link command. |
| `issue transition <id> close --reason` | Worker | Complete accountable work after proof exists. | Good. Should continue to require a reason through configured workflow transitions. |
| `issue block` | Manager/orchestrator | Record that one issue prevents another. | Good. |
| `issue unblock` | Manager/orchestrator | Remove an issue blocker relationship. | Good. |
| `issue blocked` | Reviewer | Inspect blocked work or blockers for one issue. | Good. Also useful to managers. |

## Role Guide Implication

Workers should see `list --ready`, `show`, `note`, `transition`, and `close`.
Reviewers should see `transition --options`, `blocked`, and evidence commands.
Managers should see `create`, `update`, `block`, `unblock`, and queue filters.
