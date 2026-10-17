---
created_at: "2026-10-17T01:10:57.981495535+00:00"
id: "atelier-h90a"
issue_type: "bug"
labels:
- "cli-output"
- "setup"
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
title: "Make setup claude report unchanged runs, foreign hooks and the commit step"
updated_at: "2026-10-17T01:10:57.981495535+00:00"
---

## Goal

- `atelier setup claude` prints the same paragraph on every run, including runs that changed nothing: `Installed Claude Code SessionStart and Stop hooks in …/.claude/settings.json. Commit shared settings with the change. Hooks run prime on session start and land at Stop; repeated failures warn instead of looping. Restart Claude Code to load the settings.` (crates/atelier-cli/src/commands/setup.rs:96).
- In a clone of this repository, `.claude/settings.json` holds only `bd prime` handlers for SessionStart and PreCompact. Setup adds `atelier prime --hook claude` next to `bd prime` without mentioning it, so both run every session.
- "Commit shared settings" gives no command.

## Done when

- An unchanged run prints `Hooks already installed in .claude/settings.json; nothing changed`.
- A first run lists what it added and warns about other SessionStart, Stop and PreCompact command handlers it keeps (`kept: bd prime (SessionStart, PreCompact)`).
- The output ends with `Commit: git add .claude/settings.json && git commit`, then `Restart Claude Code`.
- Tests cover the unchanged case and the foreign-handler warning.

## Context

atelier-sdb0 switches the hook to `status` and adds the CLAUDE.md `@AGENTS.md` warning. Keep the message changes compatible with that.
