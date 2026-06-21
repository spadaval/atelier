---
created_at: '2026-06-20T15:10:31.290913068+00:00'
id: atelier-e146
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-4h62
    type: advances
  - kind: issue
    id: atelier-hdff
    type: advances
  - kind: issue
    id: atelier-oc4x
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-20T22:04:29.028774912+00:00'
status: closed
title: Command surface cutting pass
updated_at: '2026-06-20T22:04:29.028774912+00:00'
---

## Description

Reduce Atelier's product surface by consolidating duplicate commands into stronger domain views, deleting removed implementation paths, and tightening module boundaries before adding more features.

## Outcome

### Constraints

- Favor fewer powerful commands over narrow special-case namespaces.
- Do not add aliases, staged deprecations, or compatibility shims for removed commands.
- Keep each removal backed by help text, command-audit docs, tests, and observable rejection behavior.

### Risks

- Removing command surface can strand hidden tests, stale docs, or role guidance if drift checks are incomplete.
- Module-boundary cleanup can expand if it starts before removed surfaces are fully deleted.

## Evidence

- Manual check: Root help and command-audit agree on the supported command surface.
- Manual check: Removed commands fail as unknown commands without compatibility guidance.
- Manual check: Focused integration tests cover replacement workflows, type-aware issue views,
  removed command behavior, and retained command status/help output.

## Notes

### Terminal Notes

- Close reason: all command-surface cutting pass epics are done, merged, and terminal checks pass

Migrated from `.atelier/missions/atelier-e146.md` as a declared mission objective issue.
