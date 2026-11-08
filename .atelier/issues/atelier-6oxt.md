---
created_at: "2026-11-08T05:52:15.560910590+00:00"
id: "atelier-6oxt"
issue_type: "task"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9s9i"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-bxs7"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Delete the moved skill and four unused Codex skills from spadaval/.agents"
updated_at: "2026-11-08T05:52:15.560910590+00:00"
---

## Goal

Remove from spadaval/.agents everything that has moved or is unused, once the skill has moved and this host's symlinks point at the new copy. That leaves it with no skills; what happens to it then is decision `atelier-rfxf`.

## Done when

- [ ] On a branch in spadaval/.agents (made with `git -C ~/projects/agents-skills worktree add`; the live checkout never changes branch), `skills/agent-factory`, `evals/agent-factory`, `skills/artifact-hub`, `skills/generate-html-plan`, `skills/pr-review-explorer` and `skills/codex-reflect` are deleted, and the README (added by `atelier-dkul`) says Agent Factory lives in spadaval/atelier, with its install line.
- [ ] Nothing on this host links into a deleted path: `find ~/.claude/skills ~/.agents/skills -maxdepth 1 -type l -exec readlink -f {} \;` lists no path under `~/projects/agents-skills/skills/` that the branch deletes.
- [ ] The branch is independently reviewed and merged into spadaval/.agents `main`; pushing waits for Sumeet.

## Scope

spadaval/.agents. Whether to retire the repository is `atelier-rfxf`, which Sumeet decides. The `firecrawl*` skills live only in `~/.agents/skills`, not in this repository, and are untouched.

## Context

Waits on this host's symlinks moving to this repository (the install loose end) and on the codex-reflect fold, so nothing live breaks and nothing useful is lost.
