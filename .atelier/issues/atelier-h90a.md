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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make setup claude report unchanged runs, foreign hooks and the commit step"
updated_at: "2026-10-17T01:10:57.981495535+00:00"
---

## Goal

`atelier setup claude` says it keeps every other hook in `.claude/settings.json` but not which, and says to commit the file without giving the command (checked 2026-11-11 with the new binary in a converted scratch clone). When this was filed, a clone of this repository held `bd prime` handlers for SessionStart and PreCompact, which then ran beside Atelier's without a mention.

## Done when

- [ ] A first run lists what it added and the other SessionStart, Stop and PreCompact command handlers it keeps (`kept: bd prime (SessionStart, PreCompact)`).
- [ ] The output ends with `Then: git add .claude/settings.json && git commit`, then `Restart Claude Code`.
- [ ] Tests cover the foreign-handler list and the last lines.

## Context

atelier-sdb0 switches the hook to `status` and adds the CLAUDE.md `@AGENTS.md` warning. Keep the message changes compatible with that.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `setup claude` already says `nothing changed` on a re-run, so that part left; the hooks it writes are `status` and `check`, not `prime` and `land`.
