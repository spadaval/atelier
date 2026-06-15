# `atelier plan`

Primary role: Manager/orchestrator.

Primary question: "How do I record, revise, link, or apply durable execution
intent?"

## Assessment

- Name: Correct.
- Documentation: Good. It should be clear that plans are durable intent, not
  private chat notes.
- Design: Mostly correct. `plan apply` is powerful and should remain framed as
  reviewed bulk record creation, not routine worker planning.
- Output hierarchy: Plan identity/status, linked targets, revision or apply
  result, created/affected record counts, next inspection commands.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `plan create` | Manager/orchestrator | Create durable plan record. | Good. |
| `plan show` | Manager/orchestrator | Inspect plan content and links. | Good. |
| `plan list` | Manager/orchestrator | Find plans by status. | Good. |
| `plan revise` | Manager/orchestrator | Record a new plan revision. | Good. |
| `plan link` | Manager/orchestrator | Attach plan intent to mission/issue/etc. | Good, but relation type should stay product vocabulary. |
| `plan apply` | Manager/orchestrator | Validate or create records from authored bulk JSON. | Good as advanced orchestration; use dry-run/validate-only in guides. |
