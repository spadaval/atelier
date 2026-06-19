# `atelier plan`

Primary role: Manager/orchestrator.

Primary question: "How do I reference execution intent that should survive the
current chat?"

## Assessment

- Name: Deferred for v1.
- Documentation: Plans are ordinary Markdown artifacts or prose referenced from
  accountable work or evidence.
- Design: First-class `.atelier/plans/` records and `atelier plan` CRUD are not
  active v1 behavior. One-shot reviewed graph creation belongs to `bundle`.
- Output hierarchy: No v1 plan command output contract. Plan paths or prose are
  inspected through the record that references them.

## Deferred Surface Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| removed `plan create` | Manager/orchestrator | Legacy attempt to create a first-class plan record. | Replacement path for durable execution intent. | Write ordinary Markdown and reference it from mission/issue/evidence, or use `bundle preview` for graph creation. | Correctly removed/deferred, but generic unknown-command errors should eventually give product guidance without adding an alias. |
| removed `plan show/list/revise/link` | Manager/orchestrator | Legacy attempt to inspect, revise, or attach plan records. | Where the plan content now lives and how to inspect it. | Inspect the referenced Markdown path, `mission show`, `issue show`, or `history`. | No v1 lifecycle state exists for plans. |
| `bundle preview/apply` | Manager/orchestrator | Replacement for authored bulk graph creation, not plan CRUD. | Bundle validation and mutation details. | Preview first, then apply with `--yes` when reviewed. | Active surface; see [bundle](bundle.md). |

## Guidance Finding

The removal is conceptually right, but known retired commands are predictable
friction points. A future CLI error can say "plans are ordinary Markdown; use
`bundle preview` for authored graph creation" without reintroducing a
compatibility command.
