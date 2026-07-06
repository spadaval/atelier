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
| `evidence attach <id> ...` | Reviewer | Reuse existing proof on another target. | Budget pressure. This is relationship mutation and may belong in the general link model. |

## Complexity Budget

`evidence record`, `evidence show`, and `evidence list` own distinct proof jobs.
`evidence attach` survives only if it is simpler than a general relationship
mutation for cross-kind proof links.

## Human Output Debt

`evidence list` currently has the clearest default-budget problem: this checkout
prints hundreds of records and includes command transcripts inline. That is
technically complete but not usable as a human browse surface.

Refresh target:

- bound the default list and state the omitted count;
- group by result, kind, target, or recency when useful;
- elide command transcripts to one human sentence with `evidence show <id>` as
  the drill-down;
- keep evidence IDs visible but secondary after the summary; and
- preserve quiet output as the composition path for IDs and status tokens.
