---
created_at: "2026-06-21T17:40:34.549734351+00:00"
id: "atelier-od9a"
issue_type: "feature"
labels:
- "evidence"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-lkz6"
  - kind: "issue"
    id: "atelier-mmhf"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Make evidence validator failures provide simple help hints"
updated_at: "2026-06-21T18:58:50.006715706+00:00"
---

## Description

Keep evidence.attached as an optional built-in validator capability, but make its failure output include a concise reason and simple help hint that callers can surface without bespoke evidence-specific status logic.

## Outcome

- evidence.attached returns a stable concise failure reason when required evidence is absent.
- evidence.attached returns a simple help hint suitable for transition options, blocked transitions, issue show/status, and root status.
- Validator result plumbing carries the hint without introducing a large structured presentation framework.
- Workflow.yaml chooses the validator and parameters; it does not need to carry detailed UI copy.

## Evidence

- Focused validator tests show evidence.attached pass/fail behavior and help hint output.
- CLI snapshot or transcript tests show blocked transition output includes the simple help hint.
- atelier lint and git diff --check pass.
