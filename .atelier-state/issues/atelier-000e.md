---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000e"
issue_type: "task"
labels:
- "fork"
- "rename"
- "spec"
- "task"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000k"
  - kind: "issue"
    id: "atelier-0012"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Rename package, binary, resources, and user-facing text for Atelier"
updated_at: "2026-06-08T19:55:23+00:00"
---


Apply the chosen CLI naming decision across Cargo metadata, binary target, command help/about text, resource paths, generated hook references, and integration tests. Keep compatibility only where the decision explicitly requires it.

## Acceptance Criteria

The main binary and help output use the chosen Atelier name; tests no longer rely on unintended Chainlink user-facing names; resource and generated paths are updated or documented as intentionally transitional; cargo test --test cli_integration passes or failures are captured in follow-up beads.
