---
created_at: "2026-10-16T23:04:24.267737941+00:00"
id: "atelier-bw0b"
issue_type: "task"
labels:
- "orientation"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-bw0b
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-7u4o"
  - kind: "issue"
    id: "atelier-deyy"
  - kind: "issue"
    id: "atelier-sdb0"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Record ADR 0022: status is the orientation view, and output is dense Markdown"
updated_at: "2026-10-16T23:05:22.980263134+00:00"
---

## Goal

Settle two linked design questions before the orientation follow-ups build on them: what a session-start command should show (live state, not static guidance), and what shape command output takes for agents (dense Markdown, not JSON). `prime` and `status` overlap today, `prime` repeats guidance that AGENTS.md already carries, and agents pipe atelier output through `head`, `grep`, and `sed` to get the views they need.

## Done when

`docs/adr/0022-status-orients-and-output-is-dense.md` is accepted by Sumeet and merged, and AGENTS.md no longer names the finished rebuild mission as the current mission.

## Scope

The ADR, plus the repository guidance it depends on: AGENTS.md, CLAUDE.md (import AGENTS.md), `.claude/settings.json` (Atelier hooks in place of a leftover `bd prime`), stale rebuild text in `docs/index.md`, `docs/product/development-setup.md` and `docs/product/cli-surface.md`, and the amendment note on ADR 0020. Product docs and code change in the follow-up issues the ADR names.

## Context

Discussed with Sumeet on 2026-10-16. Inputs: `crates/atelier-cli/src/commands/session.rs` (`prime_text`), `docs/product/human-cli-output.md`, `docs/product/sessions-and-hooks.md`, ADR 0006 and ADR 0019 (which removed `atelier man`), and a count of piped atelier commands in this project's Claude Code transcripts.
