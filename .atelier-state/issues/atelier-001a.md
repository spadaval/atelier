---
acceptance: []
created_at: "2026-06-09T17:29:00.549237358+00:00"
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
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000c"
  - kind: "issue"
    id: "atelier-000j"
  - kind: "issue"
    id: "atelier-000m"
  - kind: "issue"
    id: "atelier-000r"
  children:
  - kind: "issue"
    id: "atelier-001b"
  - kind: "issue"
    id: "atelier-001c"
  - kind: "issue"
    id: "atelier-001d"
  - kind: "issue"
    id: "atelier-001e"
  - kind: "issue"
    id: "atelier-001f"
  - kind: "issue"
    id: "atelier-001g"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "CLI surface realignment and legacy command pruning"
updated_at: "2026-06-10T02:51:33.228149103+00:00"
---

Align the public Atelier CLI with the agent-native product thesis before adding more commands. This epic prunes inherited or low-fit Chainlink surface area and establishes a clear tiering policy for core commands, compatibility aliases, integrations, and removed behavior.

## Acceptance

Public help emphasizes core tracker and workflow commands; legacy Chainlink-specific utilities are hidden, moved behind experimental or integration namespaces, or removed with compatibility notes. The result is documented and validated before worktree, domain-model, workflow, or Mission Control expansion proceeds.
