---
created_at: "2026-06-10T00:42:57.859108217+00:00"
id: "atelier-0028"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
- "migration"
- "storage"
- "task"
- "taxonomy"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0027"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T01:03:17.819291168+00:00"
status: "done"
title: "Separate issue type, record kind, and labels in canonical state"
updated_at: "2026-06-10T01:03:17.819291168+00:00"
---

## Description

Fix the current issue taxonomy behavior where Agent Factory and issue JSON infer issue_type from labels while canonical export writes every issue as issue_type: task. This makes milestone-shaped containers display as epics, turns labels into schema, and makes CLI output disagree with canonical Markdown.
An issue record with label epic is not reported as issue_type epic unless its canonical issue_type is actually epic; canonical export preserves real issue_type instead of hardcoding task; issue list/show/ready JSON agree with Markdown front matter; labels remain searchable annotations only; current milestone-shaped backlog records no longer create contradictory CLI output; tests cover import, export, rebuild, JSON output, and lint for type/label drift.
Evidence was not specified in the legacy issue record.
### Scope

- Add or preserve an explicit issue_type field in the runtime issue model and canonical Markdown projection.
- Stop deriving issue_type from labels for Agent Factory list/show/ready JSON.
- Keep labels as labels only; labels must not define schema kind or type.
- Distinguish record kind, issue_type, and relationship role in command output and JSON.
- Define transitional handling for imported Beads type labels such as epic without letting them override canonical type.
- Add lint/export/rebuild checks that catch issue_type/label drift and impossible combinations.
- Clean current repository tracker records so milestone/mission placeholder labels are explicit compatibility labels, not source-of-truth type.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
