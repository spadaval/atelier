# `atelier evidence`

Primary role: Reviewer.

Primary question: "How do I record, inspect, and reuse proof?"

## Assessment

- Name: Correct. Evidence is a first-class product concept.
- Documentation: Good. `record` should be the taught entry point; `attach`
  should be secondary.
- Design: Correct. It supports both manual summaries and command transcripts.
- Output hierarchy: Evidence ID, target, kind, result, stored command or
  artifact reference, next inspection command.

## Subcommands

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `evidence record --target issue/<id> ...` | Reviewer | Capture validation, test, review, or artifact proof. | Good. |
| `evidence show <id>` | Reviewer | Inspect one proof record. | Good. |
| `evidence list` | Reviewer | Find proof records. | Good. |
| `evidence attach <id> ...` | Reviewer | Reuse existing proof on another target. | Good as secondary surface. |
