---
created_at: "2026-06-11T18:54:48.877278641+00:00"
id: "atelier-xcy9"
issue_type: "task"
labels:
- "assignee:root"
- "config"
- "init"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-dydv"
  - kind: "issue"
    id: "atelier-vme7"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-2o5r"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Simplify atelier init to create core tracker state only"
updated_at: "2026-06-11T23:42:13.850110047+00:00"
---

## Description

Make atelier init initialize only core Atelier tracker state, not copied agent-rule bundles or Claude integration files.

Scope:
- Stop creating .atelier/rules and .atelier/rules.local during core init.
- Stop writing hook-specific config as .atelier/hook-config.json during core init.
- Create .atelier/config.toml when missing, preserving an existing tracked config.
- Create or open .atelier/state.db as local runtime state.
- Keep init idempotent and clear about what it changed.

Out of scope:
- Removing packaged resources from the crate if optional integrations still need them.
- Implementing the optional Claude integration command.

## Outcome

- Fresh init produces a small .atelier/ containing tracked config plus ignored runtime DB only.
- Existing init tests are updated to assert the reduced surface.
- No generated .atelier/rules tree appears after core init.

## Evidence

Evidence was not specified in the legacy issue record.
