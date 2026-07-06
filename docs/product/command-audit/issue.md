# `atelier issue`

Primary role: Manager/orchestrator.

Primary question: "How do I create, list, inspect, mutate, and advance
accountable issue work, including objective records that replace missions?"

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

Current issue forms for that target are:

- `issue create --issue-type mission` for mission-shaped coordination work.
- `issue list` for generic issue inventory once the current listing gap is
  fixed. It is not the mission dashboard and not the operational work queue.
- `issue show <objective-id>` for rich objective detail, hierarchy, blockers,
  evidence, affected-record context, objective health, ready and blocked work,
  proof gaps, terminal readiness, and completion gates.
- `work mission <objective-id>` for mission orchestration dashboards.
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
  parent/children, objective links, evidence, relationship context, and affected
  records that may need review. For mutations, changed fields or transition
  result first, then canonical path and next commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `issue create` | Manager/orchestrator | Create actionable issue-shaped work. | Good. Keep `--issue-type` and template behavior explicit. |
| `issue list` | Manager/orchestrator | Browse issue records with simple metadata filters. | Missing today but correct target. Keep it inventory-shaped; do not turn it into the work dashboard. |
| `issue show` | Worker | Understand the work slice, proof expectations, and relationship context. | Good. It includes issue-scoped downstream impact so operators do not leave the issue view for blast-radius context. |
| `issue transition` | Reviewer | Inspect or execute workflow gates. | Good. It belongs with issue mutation; `transition options` should be the reviewer entry point. |
| `issue update` | Manager/orchestrator | Correct issue metadata, parent, labels, type, priority. | Good. Current work is derived from canonical issue status plus checkout context, not from separate runtime ownership state. |
| `issue note` | Worker | Add durable progress, handoff, or observation context. | Good. |
| `issue link` / `issue unlink` | Manager/orchestrator | Add or remove typed issue relationships such as objective work membership. | Good. Keep relationship mutation under issue records rather than a generic root link command. |
| `issue transition <id> close --reason` | Worker | Complete accountable work after proof exists. | Good. Should continue to require a reason through configured workflow transitions. |
| `issue link` | Manager/orchestrator | Record that one issue prevents another. | Good. |
| `issue unlink` | Manager/orchestrator | Remove an issue linker relationship. | Good. |
| `issue show` | Reviewer | Inspect blocked work or blockers for one issue. | Good. Also useful to managers. |

## Role Guide Implication

Workers should see `show`, `note`, `transition`, and close transitions after
selecting assigned or ready work from `work ready`.
Reviewers should see `transition`, `issue show`, and evidence commands.
Managers should see `create`, `list`, `update`, `link`, `unlink`, and focused
work dashboards.

## Help Drift

Root help currently describes `issue` as including "list", but
`atelier issue --help` has no `list` subcommand. The product target is to add
`issue list` as a simple inventory surface. Do not compensate by making
`work queue` the generic inventory owner.

`atelier issue --help` must also avoid implying an `issue close` subcommand.
Closure is intentionally owned by `issue transition <id> close --reason`.

## Human Output Debt

The issue family is the highest-impact single-record UX target because it owns
detail views, blockers, objective status, transitions, links, and notes.

- `issue show` should keep blocker and relationship detail scannable without
  duplicating queue-style drill-down commands on every row.
- Queue summaries should avoid `key=value` count blobs in human output.
  Summaries should read as human labels and counts while quiet output stays
  terse.
- Replace opaque group labels such as `context; parent blocked` with domain
  language that says why the parent or child is being shown.
- `issue show` and `issue transition` should summarize dirty
  checkout state instead of printing every dirty path on one line, and they
  should not repeat the same path list in multiple sections.
- Recent activity in `issue show` should render as concise event sentences.
  Raw event fields belong in `history` or verbose output.
- Transition output should hide passing validators and full action/debug
  machinery by default. Show transition names, allowed/blocked state, and failed
  requirements; keep the full detail behind verbose output.
