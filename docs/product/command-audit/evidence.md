# `atelier evidence`

Primary role: Reviewer.

Primary question: "How do I record, inspect, and reuse proof?"

## Decision Record

| Operator question | Role | Product/cognitive cost | Architecture/code cost | Verdict | Next action |
| --- | --- | --- | --- | --- | --- |
| How do I record and inspect proof? | Reviewer | Low when each verb has one job and list output is bounded. | First-class evidence records and typed attachment links already have domain owners. | Simplify | Keep the four distinct jobs, bound `list`, and keep `attach` secondary to targeted `record`. |

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
| `evidence record --target issue/<id> ...` | Reviewer | Capture validation, test, review, or artifact proof. | Keep. It is the only normal proof-capture workflow. |
| `evidence show <id>` | Reviewer | Inspect one proof record. | Keep. It owns full proof and bounded transcript inspection. |
| `evidence list` | Reviewer | Find proof records. | Simplify. Default output is capped at 20 and quiet output emits matching IDs only. |
| `evidence attach <id> ...` | Reviewer | Reuse existing proof on another target. | Keep, secondary. It is the typed cross-kind reuse owner; generic issue linking intentionally rejects evidence records. |

## Complexity Budget

`evidence record`, `evidence show`, and `evidence list` own distinct proof jobs.
`evidence attach` survives because cross-kind proof reuse is evidence-domain
behavior: it validates the `validates` role, updates canonical evidence
relationships, refreshes the projection, and records issue activity. The public
generic link root is removed and `issue link` owns issue-to-issue relationships,
so moving proof reuse there would blur record kinds rather than simplify them.

## Human Output Contract

`evidence list` applies a 20-record default budget and elides command-backed
proof to a short command summary. Full output remains owned by `evidence show`.

The retained contract:

- bound the default list and state the omitted count;
- order by recency and keep result, kind, and target visible;
- elide command transcripts to one human sentence with `evidence show <id>` as
  the drill-down;
- keep evidence IDs visible but secondary after the summary; and
- preserve quiet output as the unbounded composition path for matching IDs.
