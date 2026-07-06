---
created_at: "2026-06-23T20:16:52.831565225+00:00"
id: "atelier-59vp"
issue_type: "feature"
labels:
- "cli"
- "tracker"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T22:12:20.909039194+00:00"
status: "done"
title: "Enforce mission and epic hierarchy rules"
updated_at: "2026-06-23T22:12:20.909039194+00:00"
---

## Description

Enforce the fixed hierarchy model in issue creation, issue parent updates, bundle preview/apply, rebuild/lint validation, and recovery diagnostics. Missions cannot have parents or children; epics cannot have parents and cannot be children; non-epic issue types can be standalone or direct children of epics; non-epic issue types cannot own children.

## Outcome

- Invalid mission/epic hierarchy shapes are rejected with clear fix commands, existing invalid canonical records are reported by lint/rebuild, and ordinary standalone issues remain valid.
- `atelier bundle preview` and `atelier bundle apply` reject bundle issue parents that would create mission-owned children, task-owned children, epic parents, or nested epics.
- Bundle-created mission scope uses `advances` links only; bundle input cannot express mission work through `parent`.

## Evidence

- Focused tests cover rejected mission parents, mission children, epic parents, nested epics, children under non-epic issue types, and valid standalone or epic-child ordinary issues.
- Focused tests cover `atelier bundle preview` and `atelier bundle apply` rejection for invalid hierarchy input and acceptance of mission scope expressed through `advances` links.
- Lint or rebuild validation output reports invalid canonical hierarchy records with public recovery guidance.
- `target/debug/atelier lint`, focused CLI tests, and `git diff --check` pass.
