---
acceptance: []
blocks:
- "atelier-000c"
- "atelier-000j"
- "atelier-000m"
- "atelier-000r"
created_at: "2026-06-09T17:29:00.549237358+00:00"
depends_on: []
evidence_required: []
id: "atelier-001a"
issue_type: "task"
labels:
- "assignee:root"
- "beads:type:epic"
- "cleanup"
- "cli"
- "legacy-drag"
- "spec"
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "CLI surface realignment and legacy command pruning"
updated_at: "2026-06-09T17:30:35.503542240+00:00"
---

Align the public Atelier CLI with the agent-native product thesis before adding more commands. This epic prunes inherited or low-fit Chainlink surface area and establishes a clear tiering policy for core commands, compatibility aliases, integrations, and removed behavior.

## Acceptance

Public help emphasizes core tracker and workflow commands; legacy Chainlink-specific utilities are hidden, moved behind experimental or integration namespaces, or removed with compatibility notes. The result is documented and validated before worktree, domain-model, workflow, or Mission Control expansion proceeds.
