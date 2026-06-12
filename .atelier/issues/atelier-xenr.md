---
acceptance: []
created_at: "2026-06-12T01:38:07.057899391+00:00"
evidence_required: []
id: "atelier-xenr"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "ergonomics"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Consolidate issue creation around file-first issue create"
updated_at: "2026-06-12T03:35:21.587689662+00:00"
---

Make `atelier issue create <title>` the single normal path for creating issue
records. The command should allocate a valid canonical issue ID, write the
tracked `.atelier/issues/<id>.md` file with complete required front matter,
refresh the projection, and print the issue ID plus the Markdown path an agent
should edit next.

Remove redundant issue creation commands instead of keeping compatibility
aliases. In scope: eliminate or fold `issue quick` into `issue create --work`,
eliminate or fold `issue subissue` into `issue create --parent <id>`, and ensure
help/docs present one obvious creation command.

Acceptance:

- `atelier issue create "Title"` creates a valid sparse canonical Markdown
  issue that can be edited directly.
- The default human output shows the new ID, canonical file path, and a focused
  validation command such as `atelier lint <id>`.
- Parent and active-work creation are modeled as `--parent <id>` and `--work`
  options on `issue create`.
- `issue quick` and `issue subissue` are removed from the primary command
  surface rather than retained as compatibility aliases.
- Agent Factory and repository guidance describe the workflow as: create the
  issue shell, edit the Markdown file, then lint/validate.
