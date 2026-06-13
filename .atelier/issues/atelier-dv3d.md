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
closed_at: "2026-06-13T04:24:01.173001255+00:00"
status: "done"
title: "Add command-backed evidence capture"
updated_at: "2026-06-13T04:24:01.173001255+00:00"
---

## Description

Implement command-backed recording inside the unified evidence workflow so operators can capture proof from command execution without hand-writing long summaries. The UX should support tests, CLI transcripts, and audit commands while preserving enough output to inspect later.
- An operator can run a command through the normal evidence recording surface and attach the resulting evidence to accountable work such as an implementation, validation, review, or closeout issue.
- Captured evidence records command, exit status, relevant output, result classification, and target attachments.
- Sensitive or excessive output has a documented handling rule.
- Focused CLI tests cover successful command-backed evidence recording, failed command recording, and attachment to accountable work.
- Transcript proves a real check can be captured without manually writing the evidence body.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
