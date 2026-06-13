---
created_at: "2026-06-13T02:35:56.733150569+00:00"
id: "atelier-dv3d"
issue_type: "feature"
labels:
- "cli"
- "evidence"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-rzsg"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Add command-backed evidence capture"
updated_at: "2026-06-13T02:35:56.733150569+00:00"
---

## Description

Make it easy to capture proof from command execution without hand-writing long summaries. The UX should support tests, CLI transcripts, and audit commands while preserving enough output to inspect later.

## Outcome

- An operator can run a command through an evidence capture surface and attach the resulting evidence to an issue or mission.
- Captured evidence records command, exit status, relevant output, result classification, and target attachments.
- Sensitive or excessive output has a documented handling rule.

## Evidence

- Focused CLI tests cover successful command capture, failed command capture, and attachment to a tracker target.
- Transcript proves a real check can be captured without manually writing the evidence body.
