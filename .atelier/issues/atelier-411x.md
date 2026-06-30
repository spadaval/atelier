---
created_at: "2026-06-29T20:15:57.089955580+00:00"
id: "atelier-411x"
issue_type: "epic"
labels:
- "cli"
- "complexity"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-6knt"
  - kind: "issue"
    id: "atelier-fmb7"
  - kind: "issue"
    id: "atelier-p0am"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Collapse retired command audit and stale guidance"
updated_at: "2026-06-29T20:16:42.094521639+00:00"
---

## Description

The command audit still preserves old command names, retired behavior, and stale examples in places that should be compact historical references. Retired surfaces should not read like supported manuals.

## Outcome

Retired command pages are collapsed to tombstones or short history notes, stale role-guide and product-doc references are removed or rewritten to current commands, and command-audit documentation presents the current complexity-budget decisions instead of preserving old workflow instructions.
