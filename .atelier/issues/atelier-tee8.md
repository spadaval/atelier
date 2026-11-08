---
created_at: "2026-11-08T23:41:02.959451965+00:00"
id: "atelier-tee8"
issue_type: "loose_end"
labels:
- "agent-factory"
- "needs-human"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6oxt"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Push spadaval/.agents main, so GitHub's copy drops Agent Factory and the four Codex skills"
updated_at: "2026-11-08T23:41:02.959451965+00:00"
---

## Goal

Mission atelier-g6rp's outcome says spadaval/.agents no longer holds Agent Factory. Locally that is true. The rest waits on Sumeet, because every item said pushing to spadaval/.agents waits for him.

## Done when

- [ ] Sumeet has reviewed and pushed `~/projects/agents-skills` `main` (`dfa5f3e`, 17 commits ahead of `origin/main`). It holds:
  - atelier-l58v's skill change (`c75a9b1`);
  - atelier-dkul's README and frozen notice (`aef856d`);
  - atelier-6oxt's deletion of `skills/agent-factory`, `evals/agent-factory` and the four Codex skills (`dfa5f3e`).

  For example: `git -C ~/projects/agents-skills push origin main`.
- [ ] `git ls-remote git@github.com:spadaval/.agents.git main` names `dfa5f3e` or a descendant, and its tree has no `skills/` directory.

## Context

- Each change was reviewed independently and merged locally.
- atelier-rfxf keeps the repository for future personal skills.
