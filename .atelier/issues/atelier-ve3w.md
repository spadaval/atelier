---
created_at: "2026-06-12T01:54:57.864059395+00:00"
id: "atelier-ve3w"
issue_type: "task"
labels:
- "cli"
- "output"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T03:35:45.167209738+00:00"
status: "done"
title: "Standardize mutation output around explicit change summaries"
updated_at: "2026-06-12T03:35:45.167209738+00:00"
---

## Description

Mutation commands should state exactly what changed instead of relying on
generic acknowledgements or generic next-command blocks. The output should be
short, but it should identify the record, changed fields or relationships, the
canonical file path when applicable, and any focused validation command.
- Define a common mutation acknowledgement shape for issue, mission, work,
  dependency, link, evidence, and plan mutations.
- Mutations list concrete changes such as `status: open -> closed`,
  `labels: added cli`, `parent: none -> atelier-1234`, or
  `blocked_by: added atelier-abcd`.
- Commands avoid vague success text when changed state can be named.
- Quiet output remains minimal if quiet mode is retained by the quiet-mode
  artifact-update task.
- Transcript tests prove representative mutation outputs are explicit and
  concise.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
