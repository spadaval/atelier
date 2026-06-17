# `atelier plan`

Primary role: Manager/orchestrator.

Primary question: "How do I record, revise, or link durable execution intent?"

## Assessment

- Name: Correct.
- Documentation: Good. It should be clear that plans are durable intent, not
  private chat notes.
- Design: Mostly correct. Plan commands own durable plan artifacts and links.
  One-shot reviewed graph creation belongs to `bundle`, not to plan commands.
- Output hierarchy: Plan identity/status, linked targets, revision result, and
  next inspection commands. Created-record summaries belong to bundle commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `plan create` | Manager/orchestrator | Create durable plan record. | Good. |
| `plan show` | Manager/orchestrator | Inspect plan content and links. | Good. |
| `plan list` | Manager/orchestrator | Find plans by status. | Good. |
| `plan revise` | Manager/orchestrator | Record a new plan revision. | Good. |
| `plan link` | Manager/orchestrator | Attach plan intent to mission/issue/etc. | Good, but relation type should stay product vocabulary. |
| `bundle preview/apply` | Manager/orchestrator | Validate or create records from authored bundle JSON. | Good as advanced orchestration; use preview first and require `apply --yes` for mutation. |
