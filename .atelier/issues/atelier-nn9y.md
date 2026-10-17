---
created_at: "2026-10-17T01:44:18.187502518+00:00"
id: "atelier-nn9y"
issue_type: "mission"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-08rc"
    type: "advances"
  - kind: "issue"
    id: "atelier-0vp3"
    type: "advances"
  - kind: "issue"
    id: "atelier-6tmk"
    type: "advances"
  - kind: "issue"
    id: "atelier-h90a"
    type: "advances"
  - kind: "issue"
    id: "atelier-hfze"
    type: "advances"
  - kind: "issue"
    id: "atelier-pxyz"
    type: "advances"
  - kind: "issue"
    id: "atelier-rmvb"
    type: "advances"
  - kind: "issue"
    id: "atelier-rp1c"
    type: "advances"
  - kind: "issue"
    id: "atelier-t3ko"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Make init, setup, prune and the policy view report what they did"
updated_at: "2026-10-17T01:44:18.187502518+00:00"
---

## Outcome

`init` detects the base branch, and reports what it did and the real next steps. A repository without Git, or without commits, gets a clear refusal. `setup claude` reports unchanged runs, foreign hooks and the commit step. `prune` summarises eligible versus kept artifacts one line per row, names the root that keeps each cluster, and ends with the commit step. The workflow policy has a readable view.

## Success criteria

- S1: `init` detects the base branch, reports what it did and the next steps, and refuses clearly without Git or without commits. `land_base_mismatch` names a fix (atelier-0vp3, atelier-t3ko, atelier-6tmk). Shown by CLI tests in fresh repositories.
- S2: `setup claude` reports unchanged runs, foreign hooks such as a leftover `bd prime`, and the commit step (atelier-h90a). Shown by setup tests.
- S3: `prune` summarises eligible versus kept artifacts, one line per row, and names the root keeping each cluster. It suggests `--apply` only when apply would do something, and `--apply` ends with the commit step and an exact recovery (atelier-rp1c, atelier-rmvb, atelier-pxyz, atelier-08rc). Shown by prune tests.
- S4: A command shows the workflow policy in dense Markdown (atelier-hfze). Shown by a CLI test.

## Background

Split out of atelier-o3lm on 2026-10-16 at the human's direction. Its items come from that mission's six-surface CLI audit; see atelier-o3lm's Background for the method. The output contract (atelier-o3lm S1, atelier-i2qq) and ADR 0022 (atelier-bw0b) govern how output looks. This repository's own `.claude/settings.json` still ran `bd prime` on 2026-10-16; the fix is on task/atelier-bw0b.
