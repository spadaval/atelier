---
created_at: "2026-10-17T01:10:57.964882526+00:00"
id: "atelier-yzuv"
issue_type: "task"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Stop mission issue show restating scope three times"
updated_at: "2026-10-17T01:10:57.964882526+00:00"
---

## Goal

`atelier issue show 1w8c` (a mission) lists its scope three times: seven `advances …` rows under Linked Issues, a `Buckets: active 0, ready 2, blocked 3, done 0, backlog 2` line, and Ready/Blocked Work panels (`render_objective_rollup_section`, `crates/atelier-cli/src/commands/issue.rs:151-245`). Other problems in the same view:
- `Relationships: advances 7, open blockers 2, validating evidence 0, other 14`: "other 14" is never explained.
- Validating evidence is printed twice (Relationships and Evidence Gates).
- `Recent Activity Facts` labels atelier-sdb0 `[blocked]` while Linked Issues labels it `[todo]`.
- Subissues and Impact, which can never apply to a mission, print `(none)`.
- On parents, the Subissues summary uses a different format: `1 total | status: done=1 | priority: critical=1`.

## Done when

- Mission show prints Outcome, Success criteria, strategy path, and one scope line, e.g. `Scope: 7 issues: 2 ready, 3 blocked, 2 backlog. More: atelier work mission atelier-1w8c`. Nothing else repeats `work mission`.
- Sections that can't apply to missions are omitted.
- Each item uses one status word everywhere in the view.
- The parent Subissues summary matches the work views' count wording.
- Rendering tests are updated.

## Context

Extends atelier-7u4o (brief `issue show`) and atelier-deyy (dense work rows).
