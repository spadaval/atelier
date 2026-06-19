# Deferred Checkpoint Semantics

Milestone/checkpoint semantics are deferred for v1. Atelier does not currently
have a first-class `.atelier/milestones/` record table, milestone lifecycle, or
public milestone command group.

Checkpoint intent is still useful product language. When an operator needs to
name an intermediate target state, write it in the accountable mission, epic,
issue, or evidence body. That prose may describe desired state, scope
boundaries, validation criteria, accepted evidence, and remaining risk, but it
does not create a separate durable milestone record.

## V1 Validation Destination

Validation and outcome data stays on v1 accountable records:

| Data | V1 home |
| --- | --- |
| Mission-level objective and validation expectations | Mission Markdown sections |
| Epic review and grouped-branch completion judgment | Epic issue record and evidence |
| Implementation, migration, review, or validation accountability | Issue records |
| Command transcripts, reviews, audits, screenshots, and proof summaries | Evidence records |
| Checkpoint notes or plan references | Mission, epic, issue, or evidence prose |

Workflow validators belong to `.atelier/workflow.yaml` and issue transition
evaluation. They do not attach to milestone records because milestone records
are not active v1 storage.

## Referencing Checkpoint Prose

Use explicit prose or paths instead of a milestone ID:

```text
See docs/plans/storage-cleanup.md#checkpoint-2 for the current checkpoint
criteria.
```

```markdown
## Validation

- Checkpoint: bundle apply can preview, validate, and apply issue, mission, and
  evidence records without first-class plan or milestone resources.
- Proof: evidence/atelier-x14g records the command transcripts and focused
  tests.
```

This keeps checkpoint meaning reviewable without adding a parallel completion
state or validation-data destination.

## Future Contract

A future checkpoint feature must introduce a fresh product and storage contract
directly. It must not restore the deleted legacy `milestone` command group,
reuse stale `.atelier/milestones/` assumptions, or smuggle checkpoint state into
bundle v1 resources. Until that contract exists, milestone/checkpoint references
in historical tracker records are archival context or explicitly deferred
planning notes.
