---
created_at: "2026-11-08T05:59:04.802836306+00:00"
id: "atelier-rfxf"
issue_type: "decision"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T21:52:24.804040674+00:00"
status: "done"
title: "Retire spadaval/.agents once it holds no skills?"
updated_at: "2026-11-08T21:52:24.804040674+00:00"
---

## Context

Sumeet asked to be asked before the repository is retired (2026-11-08).

## Question

After `atelier-6oxt`, spadaval/.agents holds no skills: its `main` has only agent-factory, its evals, the four Codex skills being deleted, and a stale `.skill-lock.json`. Should the repository be archived on GitHub, kept as a home for future personal skills, or deleted?

## Options

- Archive it on GitHub, with a README pointing at spadaval/atelier. Old links keep working.
- Keep it for future personal skills (for example `file-pr`, `boat`, `transcribe`, which live only under `~/.agents/skills` or `~/.claude/skills` today).
- Delete it. Old links in closed records break.

## Decision

Keep spadaval/.agents as a home for future personal skills; do not archive or delete it. atelier-6oxt still deletes Agent Factory, its evals and the four unused Codex skills (`artifact-hub`, `codex-reflect`, `generate-html-plan`, `pr-review-explorer`), so the repository holds no skills until a personal one moves in. Sumeet decided on 2026-11-08: archiving would close off a natural home for skills like `file-pr`, `boat` and `transcribe`, deleting would break links in closed records, and keeping it costs nothing.

## Report

Sumeet: keep spadaval/.agents for future personal skills; 6oxt deletes Agent Factory, its evals and the four Codex skills as planned.
