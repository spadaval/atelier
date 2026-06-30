---
created_at: "2026-06-29T20:12:28.950952487+00:00"
id: "atelier-8kv0"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Collapse review status and submit-like verbs where budget requires"
updated_at: "2026-06-29T20:12:28.950952487+00:00"
---

## Description

Review currently has separate verbs for status, show, comment, approve, and request-changes. Decide which verbs survive by operator job, not provider shape.

## Outcome

Review status/detail and submit-like actions have explicit Keep/Simplify/Fold decisions. If `review show` can answer concise status by default, `review status` is folded. If one submit-style owner is clearer, `review comment`, `review approve`, and `review request-changes` are collapsed or routed through that owner. Help and tests prove removed verbs are not preserved as compatibility aliases.
