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

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `plan create` | Manager/orchestrator | Create durable plan record. | Removed/deferred; use ordinary Markdown referenced from mission, epic, issue, or evidence prose. |
| `plan show` | Manager/orchestrator | Inspect plan content and links. | Removed/deferred; open the referenced Markdown path or inspect the accountable record. |
| `plan list` | Manager/orchestrator | Find plans by status. | Removed/deferred; plan records do not have v1 lifecycle state. |
| `plan revise` | Manager/orchestrator | Record a new plan revision. | Removed/deferred; revise the Markdown artifact or record a note/evidence explaining the plan change. |
| `plan link` | Manager/orchestrator | Attach plan intent to mission/issue/etc. | Removed/deferred; reference plan paths or prose directly from accountable record bodies. |
| `bundle preview/apply` | Manager/orchestrator | Validate or create records from authored bundle JSON. | Active advanced orchestration; use preview first and require `apply --yes` for mutation. |
