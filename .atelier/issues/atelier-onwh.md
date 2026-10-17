---
created_at: "2026-10-17T01:10:57.989900504+00:00"
id: "atelier-onwh"
issue_type: "task"
labels:
- "cli-output"
- "cross-cutting"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Use one footer heading and hint format across views"
updated_at: "2026-10-17T01:10:57.989900504+00:00"
---

## Goal

The next-step footer has seven names:
- `Next Commands` at 12 sites: relate.rs:108, work.rs:88/585, issue.rs:530/690/990, evidence.rs:595, bundle.rs:1556/1562, workflow.rs:70, human_output.rs:813
- `Next Actions` (status.rs:163)
- `Drill Down` (human_output.rs:445)
- per-row `Drill down:` in `work missions` (work.rs:249)
- `Commands` per transition (workflow.rs:371)
- `Next steps:` with shell comments (init.rs:18-20)
- `Next:` (prune.rs:120, use_cases.rs:19, session.rs:186), which also appears as `Hint: Next: atelier issue show ...` (workflow.rs:449)

Line formats differ too: `Label: command`, bare commands (the `issue link` footer prints `  atelier issue show atelier-7b6e` with no label), `command   # comment` (init), and prose (prime). Of `Drill down: atelier work mission atelier-1w8c`, the human said "accurate but odd; something like `More info: ...` would be clearer".

## Done when

- One footer heading, `Next`, is used everywhere, with `  <label>: <command>` lines.
- The per-row pointer in `work missions` reads `More info: atelier work mission <id>`.
- No `Hint: Next:` double prefix remains.
- `human-cli-output.md` names the format, and a test greps rendered output from a fixture repo for the retired headings.

## Context

The human's feedback on `atelier work missions`. Pairs with the hint-ID and state-aware-footer items.
