---
created_at: "2026-06-13T02:52:23.685770580+00:00"
id: "atelier-im60"
issue_type: "task"
labels:
- "cli"
- "diagnostics"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:27:41.930157889+00:00"
status: "done"
title: "Finish removing workflow validate from normal workflow"
updated_at: "2026-06-13T04:27:41.930157889+00:00"
---

## Description

Repair remaining docs, help, and Agent Factory drift that still makes `atelier workflow validate` look like normal closeout proof. The implementation may keep internal validator machinery, but operators should use domain surfaces such as issue transition options and mission status/closeout.

## Outcome

- Normal docs, help text, next actions, and Agent Factory guidance do not recommend `atelier workflow validate` as routine proof or next-step discovery.
- Any retained workflow validation command is hidden, relocated, or clearly labeled as advanced/internal diagnostics.
- `atelier issue transition <id> --options`, `atelier issue show <id>`, and mission status/closeout surfaces expose needed readiness information without raw validator names.

## Evidence

- Residue scan artifact lists remaining `workflow validate` references and classifies each as removed, diagnostic-only, or historical.
- Focused transcript tests prove normal workflows do not point operators to raw workflow validation.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.
