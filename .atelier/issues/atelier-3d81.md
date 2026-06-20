---
created_at: "2026-06-20T15:11:40.736905433+00:00"
id: "atelier-3d81"
issue_type: "task"
labels:
- "command-surface"
- "cutting-pass"
- "forgejo"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Collapse Forgejo provisioning flags to the supported roles workflow"
updated_at: "2026-06-20T21:16:34.564414207+00:00"
---

## Description

Review the Forgejo roles command surface and remove strange provisioning affordances that behave like hidden configuration mutation paths, especially `forgejo roles provision --write-config` if it remains callable.

## Outcome

Forgejo role setup has a small, explicit admin surface. Commands either check current configuration or perform a clearly supported provisioning workflow; no legacy write-config flag survives without product justification.

## Evidence

- `target/debug/atelier forgejo roles --help` shows only supported role commands and flags.
- Integration tests cover rejected removed Forgejo flags and the supported roles check/provision workflow.
- `docs/product/command-audit/forgejo.md` records the final command surface.
