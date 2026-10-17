---
created_at: "2026-10-17T01:10:57.980572352+00:00"
id: "atelier-bvl3"
issue_type: "feature"
labels:
- "cli-output"
- "land"
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
title: "Report what land checked and the next step when it passes"
updated_at: "2026-10-17T01:10:57.980572352+00:00"
---

## Goal

On a passing done-owner branch, `atelier land` prints:
```
Owner: atelier-43m1 [done]. In-progress work can hand off; completed claims satisfy applicable gates.
Landing checks passed. No commits or records created.
```
- The middle sentence is static boilerplate, and here it talks about in-progress work for a done owner.
- Nothing says which invariants ran. `merge --check` lists `pass review.current: …` and five other checks for the same owner.
- There is no next step such as `atelier merge atelier-43m1`.
- With uncommitted code it says `Unfinished checkout: 6 uncommitted path(s); hand off these changes honestly. Run atelier prime for current findings and proof.`, which names no paths and points at `prime`, the command being retired.
- On the base it says `Base commits: none unpushed outside recorded review.`, which is hard to parse.

## Done when

- A pass prints one line per invariant group with ✓, for example `✓ records committed`, `✓ 2 done claims hold`, `✓ review current`.
- A done owner ends with `Next: atelier merge atelier-43m1`.
- An active owner lists dirty paths (first 5, then a count) and refers to `atelier status`.
- The base line reads `No unpushed base commits need review.`
- `--quiet` still prints `passed`.

## Context

crates/atelier-cli/src/commands/session.rs:485-490, 585, and `git::check_base_commits`. The `prime` reference at session.rs:487 is not in atelier-sdb0's doc sweep list.
