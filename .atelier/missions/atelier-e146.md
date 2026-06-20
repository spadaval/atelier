---
created_at: "2026-06-20T15:10:31.290913068+00:00"
id: "atelier-e146"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4h62"
    type: "advances"
  - kind: "issue"
    id: "atelier-hdff"
    type: "advances"
  - kind: "issue"
    id: "atelier-oc4x"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "active"
title: "Command surface cutting pass"
updated_at: "2026-06-20T19:20:16.364311157+00:00"
---

## Intent

Reduce Atelier's product surface by consolidating duplicate commands into stronger domain views, deleting removed implementation paths, and tightening module boundaries before adding more features.

## Constraints

- Favor fewer powerful commands over narrow special-case namespaces.
- Do not add aliases, staged deprecations, or compatibility shims for removed commands.
- Keep each removal backed by help text, command-audit docs, tests, and observable rejection behavior.

## Risks

- Removing command surface can strand hidden tests, stale docs, or role guidance if drift checks are incomplete.
- Module-boundary cleanup can expand if it starts before removed surfaces are fully deleted.

## Validation

- Root help and command-audit agree on the supported command surface.
- Removed commands fail as unknown commands without compatibility guidance.
- Focused integration tests cover replacement workflows, type-aware issue views,
  removed command behavior, and retained command status/help output.
