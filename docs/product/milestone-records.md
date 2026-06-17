# Milestone Records

Milestone records are first-class checkpoint records. They describe a target
state that can be validated, not a work container, roadmap label, or super-epic.
Issues and epics do the work; evidence and workflow validators prove whether the
checkpoint can be accepted.

## Record Contract

A first-class milestone record must model these fields:

| Field | Meaning |
| --- | --- |
| `desired_state` | The observable repository, product, or workflow state that should be true at the checkpoint. |
| `scope` | Explicit boundaries for what is included, excluded, and deferred. |
| `validation_criteria` | Ordered criteria that define what must be proven before the checkpoint is accepted. |
| `accepted_evidence` | Evidence requirements or accepted evidence IDs that prove one or more validation criteria. |
| `completion_state` | The checkpoint's acceptance state, independent of whether contributing work items are open or closed. |
| `missions` | One or more mission IDs that the checkpoint belongs to or advances. |
| `contributing_work` | Linked epics, issues, reviews, or validation items that contribute to the checkpoint. |

The canonical record shape follows the storage contract in
[Canonical Record And Rebuild Layout](../spec/storage/export/rebuild/canonical-layout.md):
milestone records live under `.atelier/milestones/<record-id>.md`, IDs
use the project-scoped random record ID format, ID arrays are sorted lexically,
and text arrays preserve author order.

## Relationships

Milestones sit between mission intent and work execution:

```text
mission has_checkpoint milestone
epic contributes_to milestone
issue contributes_to milestone
evidence validates milestone.validation_criteria[N]
workflow transition uses validator
```

`missions` and `contributing_work` are links, not hierarchy that turns a
milestone into a parent work queue. A mission may have multiple checkpoint
records, and the same milestone may advance more than one mission when the
checkpoint is genuinely shared.

Contributing work links should point to accountable records that can be claimed,
reviewed, validated, and closed. A milestone is accepted because its
`validation_criteria` have sufficient evidence and the relevant workflow
transition validators pass, not because every possible related item has been
nested under it.

## Validation

Milestones own validation criteria. Workflow policy owns transition checks.

The milestone record defines what must be true. Evidence records prove what was
observed. Workflow validators decide whether a transition is allowed under the
repository's configured workflow policy. This keeps checkpoint meaning durable
and reviewable while keeping process enforcement configurable.

A workflow may require validators such as `evidence_attached`,
`validation_criteria_satisfied`, `no_open_blockers`, or `no_blocking_lints`
before a milestone acceptance or terminal transition can complete. Those
validators belong in the configured workflow policy or the workflow
implementation contract, not as milestone-attached transition checks.

Milestone verification should therefore answer three questions:

1. Which `validation_criteria` does each evidence record validate?
2. Do the accepted evidence records prove the desired checkpoint state within
   the stated scope boundaries?
3. Did the configured workflow validators allow the acceptance or terminal
   transition?

## Completion State

`completion_state` is checkpoint state, not an aggregate issue status. It should
communicate whether the milestone is proposed, active, accepted, superseded, or
abandoned according to the configured workflow vocabulary. The exact state names
may be repository-configured, but the state must describe the checkpoint record
itself.

Open contributing work does not automatically make a milestone incomplete if
the remaining work is out of scope, deferred, or linked to a later checkpoint.
Closed contributing work does not automatically complete a milestone without
accepted evidence for the validation criteria and successful workflow
validation.

## Command Surface

Milestone/checkpoint semantics are deferred for v1 and are not a validation-data
destination for bundle apply. A future dedicated checkpoint command or record
surface must target this record contract directly rather than restoring the
deleted legacy `milestone` command group or smuggling milestones into the v1
bundle resource set.
