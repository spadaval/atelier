---
created_at: "2026-10-17T01:10:57.987773924+00:00"
id: "atelier-5zta"
issue_type: "task"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Put each work section's count in its heading"
updated_at: "2026-10-17T01:10:57.987773924+00:00"
---

## Goal

`work mission` puts all counts on one line above every table (`Work  : active 1, ready 1, 1 need a human, 1 context, blocked 5, done 0, backlog 1`, crates/atelier-cli/src/commands/work.rs:401-415), with mixed wording ("ready 1" next to "1 need a human"); the reader matches numbers to sections lower down, and summary panels stop at 8 rows so counts and rows disagree. Repo-wide `work ready`, `work blocked`, `work human` and `work all` have no counts at all; an astromech agent ran `atelier work ready 2>&1 | grep -cE '^\s+ready '` to count.

## Done when

- Section headings carry their count: `Ready (1)`, `Blocked (6)`, `Needs a human (1)`, `Done (12)`.
- The `Work:` summary line is gone from `work mission` and `work parent`.
- `work ready` heading counts agree with `status` ("Ready work: 6") on the same records; a test asserts it.

## Context

Reworks the same rendering as atelier-deyy.
