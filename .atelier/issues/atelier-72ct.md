---
created_at: "2026-06-14T02:51:53.262125535+00:00"
id: "atelier-72ct"
issue_type: "task"
labels:
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Document record-kind command boundaries"
updated_at: "2026-06-14T08:27:39.416885998+00:00"
---

## Description

Add a compact matrix that explains which commands accept mission, issue,
evidence, plan, and milestone IDs after the record-specific command surfaces
are stable. Include the normal blocker, mission work link, evidence attachment,
plan relationship, graph, and note boundaries.

## Outcome

Mission IDs, issue IDs, and evidence IDs have unambiguous command families.
Wrong-kind examples point to the correct surface without recommending generic
`atelier link`, top-level `dep`, or generic `note add`.

## Evidence

- File diff shows docs include the matrix and at least one mission-vs-issue
  example.
- Examples use current help for record-specific blocker, mission link, evidence
  attachment, graph, and note commands.
- `git diff --check` and `atelier lint` pass.
