---
created_at: "2026-06-30T17:48:20.061197566+00:00"
id: "atelier-otth"
issue_type: "bug"
labels:
- "bug"
- "evidence"
- "tracker"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lz6a"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Bug: Issue-to-issue validates links are accepted but not handled as validation"
updated_at: "2026-06-30T17:48:20.061197566+00:00"
---

## Description

Issue-to-issue `validates` links currently round-trip as generic typed links, but mission/objective rollups and evidence gates only treat `validates` as meaningful when one side is an evidence record. This makes a validation issue linked with `validates` appear as `other` while the rollup reports `validates 0`, which is misleading.

## Outcome

- Decide and enforce the relation contract: `validates` should probably be evidence-only.
- Reject or lint issue-to-issue `validates` links with clear guidance.
- Update rollup wording so evidence counts cannot be confused with validation issue links.
- Migrate existing issue-to-issue `validates` links to the chosen model or document any intentional exception.
