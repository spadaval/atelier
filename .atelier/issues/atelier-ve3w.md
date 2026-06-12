---
acceptance: []
created_at: "2026-06-12T01:54:57.864059395+00:00"
evidence_required: []
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
status: "open"
title: "Standardize mutation output around explicit change summaries"
updated_at: "2026-06-12T01:54:57.864059395+00:00"
---

Mutation commands should state exactly what changed instead of relying on
generic acknowledgements or generic next-command blocks. The output should be
short, but it should identify the record, changed fields or relationships, the
canonical file path when applicable, and any focused validation command.

Acceptance:

- Define a common mutation acknowledgement shape for issue, mission, work,
  dependency, link, evidence, and plan mutations.
- Mutations list concrete changes such as `status: open -> closed`,
  `labels: added cli`, `parent: none -> atelier-1234`, or
  `blocked_by: added atelier-abcd`.
- Commands avoid vague success text when changed state can be named.
- Quiet output remains minimal if quiet mode is retained by the quiet-mode
  decision issue.
- Transcript tests prove representative mutation outputs are explicit and
  concise.
