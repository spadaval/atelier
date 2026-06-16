---
created_at: "2026-06-15T21:39:49.959113019+00:00"
id: "atelier-3yoa"
issue_type: "feature"
labels:
- "cli"
- "evidence"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make issue creation markdown-first"
updated_at: "2026-06-16T14:53:50.186028980+00:00"
---

## Description

Make issue creation a markdown-first scaffold flow. The create command should make a durable issue file with the requested metadata, print the file path and next commands, and expect agents to edit the Markdown sections directly for substantial issue bodies.

## Outcome

- `atelier issue create` no longer accepts a large free-form description body that encourages agents to squeeze structured work into shell arguments.
- New issues are created as Markdown scaffolds with `Description`, `Outcome`, and `Evidence` sections that agents can edit directly.
- Create output highlights the issue file path, `atelier issue show <id>`, and `atelier lint <id>` as the next steps for full specification.
- Status and lint surfaces make under-specified issue sections visible before the issue is treated as ready execution work.
- The flow stays non-interactive; Atelier must not launch an editor or require prompt-driven issue entry.

## Evidence

- CLI transcript creates an issue, edits the generated Markdown sections, runs `atelier lint <id>`, and shows the populated sections through `atelier issue show <id>`.
- Focused CLI tests cover create help without `--description`, scaffold file creation, next-command output, and lint/status behavior for under-specified records.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant cargo tests pass.
