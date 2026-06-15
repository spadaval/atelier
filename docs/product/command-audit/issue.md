# `atelier issue`

Primary role: Manager/orchestrator.

Primary question: "How do I create, inspect, mutate, and advance accountable
issue work?"

`issue` is intentionally shared by roles. The root noun is correct because the
command owns issue records and issue workflow state, not a single user persona.
`issue create` and `issue transition` belong in this family because both are
issue-state operations.

## Assessment

- Name: Correct. The noun maps to the durable accountability unit.
- Documentation: Needs role-aware examples. Generic `issue --help` is accurate
  but does not tell a worker which subset matters.
- Design: Mostly correct. The family is broad, but cohesion is still issue
  state and issue relationships.
- Output hierarchy: For reads, current state and blockers before metadata. For
  mutations, changed fields or transition result first, then canonical path and
  next commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `issue create` | Manager/orchestrator | Create actionable issue-shaped work. | Good. Keep `--issue-type` and template behavior explicit. |
| `issue list` | Worker | Find candidate or assigned work. | Good, but role guides should prefer `--ready`, `--blocked`, and exact status/category examples. |
| `issue show` | Worker | Understand the work slice and proof expectations. | Good. |
| `issue transition` | Reviewer | Inspect or execute workflow gates. | Good. It belongs with issue mutation; `--options` should be the reviewer entry point. |
| `issue update` | Manager/orchestrator | Correct issue metadata, parent, labels, type, priority. | Good. Hidden `--claim` should remain hidden or be removed if active work replaces claims. |
| `issue note` | Worker | Add durable progress, handoff, or observation context. | Good. |
| `issue close` | Worker | Complete accountable work after proof exists. | Good. Should continue to require a reason. |
| `issue block` | Manager/orchestrator | Record that one issue prevents another. | Good. |
| `issue unblock` | Manager/orchestrator | Remove an issue blocker relationship. | Good. |
| `issue blocked` | Reviewer | Inspect blocked work or blockers for one issue. | Good. Also useful to managers. |

## Role Guide Implication

Workers should see `list --ready`, `show`, `note`, `transition`, and `close`.
Reviewers should see `transition --options`, `blocked`, and evidence commands.
Managers should see `create`, `update`, `block`, `unblock`, and queue filters.
