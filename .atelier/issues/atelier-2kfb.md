---
created_at: "2026-06-21T16:37:30.767152802+00:00"
id: "atelier-2kfb"
issue_type: "feature"
labels:
- "cleanup"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-76j0"
  - kind: "issue"
    id: "atelier-f9ci"
  - kind: "issue"
    id: "atelier-y3fj"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T19:15:15.443640677+00:00"
status: "done"
title: "Delete mission root namespace and mission command shims"
updated_at: "2026-06-21T19:15:15.443640677+00:00"
---

## Description

Remove the mission command module as a public command namespace instead of preserving aliases.

## Outcome

- `atelier mission ...` is not visible in root help and removed forms fail as unknown commands or invalid subcommands.
- Replacement guidance in normal command output points to `atelier issue ...`, `atelier status`, `atelier history`, and `atelier evidence ...` as appropriate.
- No compatibility alias, staged deprecation, or fallback reader is added.

## Evidence

- Negative CLI tests cover old mission commands.
- `rg "atelier mission" docs AGENTS.md .agents crates/atelier-cli/src crates/atelier-cli/tests` finds only migration history or explicit removed-command tests.
