---
created_at: "2026-10-17T01:10:57.990521803+00:00"
id: "atelier-91w1"
issue_type: "task"
labels:
- "cli-output"
- "cross-cutting"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-91w1
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:11:46.540060866+00:00"
status: "done"
title: "Replace internal jargon in output with CONTEXT.md terms"
updated_at: "2026-10-17T08:11:46.540060866+00:00"
---

## Goal

Output uses terms that CONTEXT.md doesn't define:
- `check` prints "Lint passed." / "Lint found 1 issue(s):" / "Lint failed with 1 finding(s)" (issue.rs:1032-1042). There, "issue(s)" means findings, not tracker issues.
- `check atelier-7b6e` reports another record's failure under `(canonical):`.
- Land errors stack two codes and repeat the advice: `land_records_uncommitted: canonical_snapshot_uncommitted: .atelier/issues/atelier-7b6e.md path, bytes or mode differs from pinned HEAD; commit canonical records before continuing` followed by `Commit canonical records and policy with the work they describe, then retry.`
- issue show has "Objective Rollup", "Buckets", "Recent Activity Facts", and "Relationships: advances 7, open blockers 2, validating evidence 0, other 14".
- `prune` has "Canonical Records".
- `issue create --status` help says "todo-category status".
- Snake_case codes prefix about 40 errors (start_/land_/merge_/workflow_) but not others ("Issue X was not found").

## Done when

- User-facing text says "records", "check", "finding" and "mission" per CONTEXT.md. "Lint" and "canonical" don't appear in output except in codes.
- Each error states its failure once, then one recovery step.
- One rule, written into human-cli-output.md, says whether errors carry a code prefix, and it is applied everywhere.
- A grep test over rendered fixtures enforces this.

## Close Reason

Output uses CONTEXT.md terms (check, finding, records, mission, parent). lint and canonical appear only in codes and config keys. land reports one code and one Next step instead of stacking canonical_snapshot_uncommitted. The code rule is in human-cli-output.md Errors, and output_terms.rs greps rendered output to enforce it.
