---
created_at: "2026-06-19T19:39:51.356083457+00:00"
id: "atelier-rgpl"
issue_type: "feature"
labels:
- "prune"
- "records"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iq7f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Archive or compact eligible canonical records"
updated_at: "2026-06-19T19:39:51.356083457+00:00"
---

## Description

Implement the v1 behavior for reducing canonical tracked record accumulation
after the contract decides whether eligible records are archived, compacted, or
only reported.

## Outcome

- The implementation follows the documented v1 policy for closed or archived
  issues, missions, evidence records, and activity sidecars.
- Required proof for non-archived work remains protected.
- Any archive or compaction output is deterministic, rebuildable where required,
  and validated by lint or doctor checks.
- Destructive record removal remains an explicit maintenance action and is not
  hidden behind routine pruning unless the contract specifically allows it.

## Evidence

- Focused tests cover protected required evidence, eligible terminal records,
  deterministic archive or compaction output, and lint/doctor behavior after
  pruning.
- Evidence record includes before/after file summaries and residual audit risks.
