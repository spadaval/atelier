---
created_at: "2026-11-08T05:52:15.560910590+00:00"
id: "atelier-6oxt"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-6oxt
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
status: "in_progress"
title: "Delete the moved skill and four unused Codex skills from spadaval/.agents"
updated_at: "2026-11-08T23:06:13.433095439+00:00"
---

## Goal

Remove from spadaval/.agents everything that has moved or is unused, once the skill has moved and this host's symlinks point at the new copy. That leaves it with no skills; what happens to it then is decision `atelier-rfxf`.

## Done when

- [x] On a branch in spadaval/.agents (made with `git -C ~/projects/agents-skills worktree add`; the live checkout never changes branch), `skills/agent-factory`, `evals/agent-factory`, `skills/artifact-hub`, `skills/generate-html-plan`, `skills/pr-review-explorer` and `skills/codex-reflect` are deleted, and the README (added by `atelier-dkul`) says Agent Factory lives in spadaval/atelier, with its install line.
- [x] Nothing on this host links into a deleted path: `find ~/.claude/skills ~/.agents/skills -maxdepth 1 -type l -exec readlink -f {} \;` lists no path under `~/projects/agents-skills/skills/` that the branch deletes.
- [ ] The branch is independently reviewed and merged into spadaval/.agents `main`; pushing waits for Sumeet.

## Scope

spadaval/.agents. Whether to retire the repository is `atelier-rfxf`, which Sumeet decides. The `firecrawl*` skills live only in `~/.agents/skills`, not in this repository, and are untouched.

## Context

Waits on this host's symlinks moving to this repository (the install loose end) and on the codex-reflect fold, so nothing live breaks and nothing useful is lost.

## Report

spadaval/.agents branch `atelier-6oxt` (worktree `~/projects/agents-skills-wt/atelier-6oxt`, commit `99381f9` on main `aef856d`) deletes `skills/agent-factory`, `evals/agent-factory`, `skills/artifact-hub`, `skills/generate-html-plan`, `skills/pr-review-explorer` and `skills/codex-reflect` (187 files), and rewrites the README: the repository is kept for future personal skills (atelier-rfxf), Agent Factory lives in spadaval/atelier, with its `npx skills add` install line. The tree left is `README.md`, `.gitignore` and `.skill-lock.json`.

- **Links:** `find ~/.claude/skills ~/.agents/skills -maxdepth 1 -type l -exec readlink -f {} \;` lists nothing under `~/projects/agents-skills/skills/`. Agent Factory resolves to `~/projects/atelier-skill/skills/agent-factory` (atelier-bxs7).
- **Whitespace:** `git diff --check HEAD^ HEAD` is clean.
- **Left alone:** `.skill-lock.json` is stale (it names skills this repository never held), outside this item's Scope. Pushing waits for Sumeet; the live checkout `~/projects/agents-skills` stays on `main`.

