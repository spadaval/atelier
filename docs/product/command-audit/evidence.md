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

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `evidence record --target issue/<id> ...` | Reviewer, validator, worker | Capture manual validation; record command transcript; attach review/test/artifact proof to accountable work. | Evidence ID, target, kind, result/success, command or artifact reference, producer, and storage path. | Run `atelier evidence show <id>`, `atelier issue transition <id> --options`, or `atelier history --issue <id>`. | Strong fit. Help includes manual and command-backed examples; role guides should prefer command-backed proof where the claim is a command result. |
| `evidence show <id>` | Reviewer, validator | Inspect a proof record; verify command output or artifact; classify proof strength. | Target, scope, kind, result, summary, command transcript, artifacts, producer, residual risks. | Run `atelier issue show <target>`, inspect artifacts, or record follow-up validation. | Good drill-down. Output should include a next command when the record is attached to an issue. |
| `evidence list [--status <status>]` | Reviewer, manager/orchestrator | Find proof records for audit; inspect recent validation; locate failed/deferred evidence. | Evidence IDs, target, kind/status/result, producer, age, and enough summary to choose a record. | Run `atelier evidence show <id>` or `atelier history --issue <id>`. | Good, but bounded summaries and drill-down commands matter for scanability. |
| `evidence attach <id> <kind> <id>` | Reviewer, validator | Reuse existing proof on another accountable target; migrate evidence linkage; attach audit proof to a validation issue. | Existing evidence ID, new target, attachment role, and whether target is valid. | Run `atelier evidence show <id>` or `atelier issue transition <target> --options`. | Correctly secondary. New proof should usually be recorded at the issue that made or validated the claim. |

## Guidance Finding

Evidence is the proof surface, but review artifacts and evidence are distinct.
After `show` and `list`, output should orient the operator back to the
accountable issue or transition gate so proof does not become disconnected from
the work it supports.
