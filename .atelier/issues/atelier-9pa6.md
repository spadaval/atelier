---
created_at: "2026-10-17T01:10:57.962367692+00:00"
id: "atelier-9pa6"
issue_type: "feature"
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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Warn when a done transition runs on the base branch"
updated_at: "2026-10-17T01:10:57.962367692+00:00"
---

## Goal

On master in a scratch clone, `atelier issue transition 43m1 start` and then `close --reason "…"` both succeed silently. `docs/product/cli-surface.md` says "Done claims belong in committed work branch records", atelier-43m1's own note says "done claims may not be made on master", and `atelier land` refuses them later. The agent finds out only after committing. That is how the 1620b534 incident started: the land refusal was masked by a `| tail` and unreviewed records were pushed.

## Done when

- A done-category transition on the policy base branch, for a non-decision item, is refused (or warns on stderr) with `done claims belong on <owner branch>; run atelier start <id> first`.
- Decisions keep their documented base-branch exemption (`decide` on the base).
- CLI tests cover a task closed on the base (refused or warned), a task closed on its owner branch (unchanged), and a decision decided on the base (unchanged).

## Unknowns

Refuse or warn. Refusing matches `land`; warning keeps the transition usable in repositories without a branch policy.
