---
created_at: "2026-06-17T20:03:41.179345470+00:00"
id: "atelier-mrj5"
issue_type: "task"
labels:
- "bundle"
- "implementation"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Make bundle apply atomic, previewable, and workflow-aware"
updated_at: "2026-06-17T22:07:19.687192982+00:00"
---

## Description

Make bundle apply safe enough for an advanced declarative mutation command. The
current implementation validates up front but then writes sequentially, while
the input contract claims atomic behavior. It also defaults issues through a
parallel hard-coded status path instead of using workflow policy.

## Outcome

- Preview shows deterministic record and relationship actions without
  mutating canonical files.
- Mutating apply requires explicit confirmation such as `--yes`.
- Apply writes through a staged transaction or equivalent mechanism so failure
  leaves canonical tracker records unchanged.
- Issue status defaults and accepted statuses come from workflow policy.
- The command reports created IDs and relationship counts in bounded human
  output.
- Optional consume/archive behavior never runs unless apply succeeds.

## Evidence

- Tests inject a mid-apply failure and prove canonical files are unchanged.
- Tests prove workflow initial status and invalid status handling match
  `.atelier/workflow.yaml`.
- Command transcript shows bundle preview and apply output for a representative
  file.
