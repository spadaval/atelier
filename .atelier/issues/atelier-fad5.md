---
created_at: "2026-10-30T06:40:10.736546729+00:00"
id: "atelier-fad5"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let merge push the base it integrated, and stop recommending piped gates"
updated_at: "2026-10-30T06:40:10.736546729+00:00"
---

## Goal

After a gated merge, agents still chain commands to publish it: `merge` ends `Next: atelier land, then git push.`, and docs/product/human-cli-output.md (Verdict line) says `| tail -1` reads the verdict. In the 2026-10-15 to 10-29 transcripts, 276 of 399 `land`, `merge` and `check` runs were piped through `tail`, `head` or `grep`, and pushes, commits and branch deletions ran after refused gates. Give `merge` a `--push` that, after a successful integration, runs the same base checks and push as `land --push`, so integrating and publishing is one command whose exit status covers both.

## Done when

Integration tests with a bare origin show:
- `merge <id> --push` on an approved branch moves origin's base to the merge commit and ends `merge: passed, pushed to origin/<base>`, exit 0;
- with a refused gate, origin's base is unchanged, the verdict says nothing was pushed, and the exit status is non-zero;
- when the integration succeeds but the push fails, the exit status is non-zero and the verdict says the merge happened locally and nothing was pushed.
A plain `merge` success line names `--push` in its Next. No file under docs/ recommends piping a gate's output to read its verdict (`git grep -n "tail -1" docs/` shows no such advice). A receipt on atelier-ll3p judges criterion one-command-publish with a captured run of these tests.

## Scope

`crates/atelier-cli/src/commands/merge.rs`, the `land --push` code it reuses in `commands/session.rs`, and docs/product/human-cli-output.md, reviews-and-merge.md and sessions-and-hooks.md. Worktree removal stays as it is: merge already detaches the work checkout and prints the `git worktree remove` command.

## Context

- `land --push` contract: docs/product/human-cli-output.md, Verdict line.
- atelier-p1vz recounts piping on ordinary sessions; it measures, this item changes the product.
