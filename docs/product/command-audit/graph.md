# `atelier graph`

Primary role: Manager/orchestrator.

Primary question: "What depends on this record, and how does the work hierarchy
fit together?"

## Assessment

- Name: Correct for relationship inspection, though it is more abstract than
  `issue` or `mission`.
- Documentation: Good if it stays tied to impact and hierarchy rather than raw
  graph internals.
- Design: Correct. This is a cross-record relationship read surface.
- Output hierarchy: Starting record, bounded relationship set, status/blocker
  cues, then `mission show`, `issue show`, or `issue blocked`.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `graph impact <id>` | Manager/orchestrator | Understand downstream consequences before mutation or closeout. | Good. |
| `graph tree [--compact]` | Manager/orchestrator | Inspect mission and issue hierarchy. | Good. `--compact` should be the role-guide default. |
