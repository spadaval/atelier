---
created_at: "2026-06-17T22:42:35.720640314+00:00"
id: "atelier-g9bu"
evidence_type: "validation"
captured_at: "2026-06-17T22:42:33.516313948+00:00"
command: "bash -lc 'set -euo pipefail\nrg -n \"first-class plan|first-class milestone|\\\\.atelier/plans|\\\\.atelier/milestones|atelier plan|atelier milestone|plan create|plan show|milestone create|plan records|milestone records|plans,|milestones,|plans\\\\[|milestones\\\\[|plan_ids|milestone_ids|plan_drift|linked milestones|linked plans|plans and milestones|milestones and plans\" SPEC.md CONTEXT.md docs/product docs/architecture docs/spec || true\ntarget/debug/atelier lint atelier-uwb6\ntarget/debug/atelier lint\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-uwb6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uwb6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Plan/milestone removal docs contract validation: active product/spec/architecture docs now only contain deferred, removed-command, provenance, or historical references; scoped lint, full lint, and whitespace checks pass."
updated_at: "2026-06-17T22:42:39.794452962+00:00"
---

## Summary

Plan/milestone removal docs contract validation: active product/spec/architecture docs now only contain deferred, removed-command, provenance, or historical references; scoped lint, full lint, and whitespace checks pass.

## Command

```console
bash -lc 'set -euo pipefail
rg -n "first-class plan|first-class milestone|\\.atelier/plans|\\.atelier/milestones|atelier plan|atelier milestone|plan create|plan show|milestone create|plan records|milestone records|plans,|milestones,|plans\\[|milestones\\[|plan_ids|milestone_ids|plan_drift|linked milestones|linked plans|plans and milestones|milestones and plans" SPEC.md CONTEXT.md docs/product docs/architecture docs/spec || true
target/debug/atelier lint atelier-uwb6
target/debug/atelier lint
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 5168
Truncated: yes

```text
docs/spec/bundle/schema.md:105:First-class plans and milestones are not legal v1 bundle resources. Plans are
docs/spec/storage/export/rebuild/canonical-layout.md:522:`.atelier/plans/` and `.atelier/milestones/` are not active v1 canonical record
docs/spec/storage/export/rebuild/canonical-layout.md:525:evidence. They are referenced by path or text, not by a first-class plan or
docs/spec/storage/export/rebuild/canonical-layout.md:528:If a later feature reintroduces first-class plan or checkpoint records, it must
docs/product/milestone-records.md:4:have a first-class `.atelier/milestones/` record table, milestone lifecycle, or
docs/product/milestone-records.md:26:evaluation. They do not attach to milestone records because milestone records
docs/product/milestone-records.md:42:  evidence records without first-class plan or milestone resources.
docs/product/milestone-records.md:54:reuse stale `.atelier/milestones/` assumptions, or smuggle checkpoint state into
docs/product/cli-surface.md:525:| Reference execution plans | repository Markdown path or prose inside a mission, epic, issue, or evidence record | edit the accountable Markdown record or attach evidence that names the plan path | Plans are ordinary Markdown artifacts in v1. They are not `.atelier/plans/` records and do not replace issue blockers or mission work links. |
docs/product/cli-surface.md:528:| Inspect first-class evidence records | evidence ID | `atelier evidence show <evidence-id>` | Evidence records are supporting artifacts. Issue commands should reject their IDs with corrective wrong-kind guidance. Plan and milestone records are deferred v1 concepts. |
docs/product/work-model.md:15:  there is no active first-class milestone record table.
docs/architecture/markdown-first-record-store.md:154:| Forbidden | Escaped mission `data` payloads, front matter keys such as `constraints`, `risks`, `validation`, `work`, `plans`, `milestones`, `evidence`, `blockers`, or `terminal_notes`, and any second relationship surface for work, blockers, plans, checkpoints, or evidence. Mission prose may reference plan/checkpoint Markdown by path, but must not become a shadow graph. |
docs/architecture/markdown-first-record-store.md:164:`.atelier/plans/` directory, plan status lifecycle, or `plans.*` projection
docs/architecture/markdown-first-record-store.md:188:or evidence, but there is no `.atelier/milestones/` directory, milestone
docs/architecture/markdown-first-record-store.md:229:| `.atelier/plans/` | Plan | Deferred | No active v1 plan record table exists; planning intent is ordinary Markdown or prose referenced from accountable records. |
docs/architecture/markdown-first-record-store.md:230:| `.atelier/milestones/` | Milestone | Deferred | No active v1 milestone record table exists; checkpoint intent is ordinary Markdown or prose referenced from accountable records. |
docs/architecture/markdown-first-record-store.md:405:| Plan create/revise/link | Removed/deferred | V1 plans are ordinary Markdown artifacts or prose references, not `.atelier/plans/` records. |
docs/architecture/markdown-first-record-store.md:486:evidence records are durable and rebuildable; first-class plan and milestone
docs/architecture/markdown-first-record-store.md:502:   keep first-class plan and milestone records deferred until a new contract
docs/product/command-audit/plan.md:1:# `atelier plan`
docs/product/command-audit/plan.md:13:- Design: First-class `.atelier/plans/` records and `atelier plan` CRUD are not
docs/product/command-audit/plan.md:22:| `plan create` | Manager/orchestrator | Create durable plan record. | Removed/deferred; use ordinary Markdown referenced from mission, epic, issue, or evidence prose. |
docs/product/command-audit/plan.md:23:| `plan show` | Manager/orchestrator | Inspect plan content and links. | Removed/deferred; open the referenced Markdown path or inspect the accountable record. |
docs/product/command-audit/plan.md:24:| `plan list` | Manager/orchestrator | Find plans by status. | Removed/deferred; plan records do not have v1 lifecycle state. |
docs/architecture/provenance
```

## Stderr

Bytes: 0
Truncated: no

```text
```

