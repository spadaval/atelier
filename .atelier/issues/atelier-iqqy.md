---
acceptance: []
created_at: "2026-06-12T01:54:57.152137733+00:00"
evidence_required: []
id: "atelier-iqqy"
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
title: "Print canonical file paths in issue create and show output"
updated_at: "2026-06-12T01:54:57.152137733+00:00"
---

Issue creation and inspection should make the file-first workflow obvious.
Every issue creation path that remains after command consolidation should print
the canonical Markdown path for the created issue, and `issue show` should also
print the path for the displayed issue.

Acceptance:

- `atelier issue create "Title"` prints the new issue ID and canonical
  `.atelier/issues/<id>.md` path.
- Parented and active-work create modes print the same path information.
- `atelier issue show <id>` includes the canonical Markdown path in a stable,
  scan-friendly location.
- Output points agents toward editing the Markdown file and validating with
  `atelier lint <id>`.
- Transcript tests cover create, create with parent, create with work, and
  show output.
