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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let merge push the base it integrated, and stop recommending piped gates"
updated_at: "2026-11-07T21:03:31.900323926+00:00"
---

## Goal

Merge is the lifecycle's last step (atelier-jnrh), but agents still chain commands to publish it: a successful `atelier merge` ends `Next: atelier land, then git push.` (crates/atelier-cli/src/commands/merge.rs), and docs/product/human-cli-output.md (Verdict line) says `| tail -1` reads the verdict. In the 2026-10-15 to 10-29 transcripts, 276 of 399 `land`, `merge` and `check` runs were piped through `tail`, `head` or `grep`, and pushes, commits and branch deletions ran after refused gates; one such chain pushed unreviewed docs to master (incident 1620b534). Give `merge` a `--push` that, after a successful integration, runs the same base checks and push as `land --push`, so integrating and publishing is one command whose exit status covers both.

## Done when

Integration tests with a bare origin show:
- [ ] `merge <id> --push` on an approved branch moves origin's base to the merge commit and ends `merge: passed, pushed to origin/<base>`, exit 0, and `merge --push` with no ID does the same on an approved light-path branch.
- [ ] With a refused gate, origin's base is unchanged, the verdict says nothing was pushed, and the exit status is non-zero.
- [ ] When the integration succeeds but the push fails, the exit status is non-zero and the verdict says the merge happened locally and nothing was pushed.
- [ ] A plain `merge` success line names `--push` in its Next.
- [ ] No file under docs/product recommends piping a gate's output to read its verdict (`git grep -n "tail -1" docs/product` shows no such advice).

## Scope

crates/atelier-cli/src/commands/merge.rs, the `land --push` code it reuses in commands/session.rs, and docs/product/human-cli-output.md, reviews-and-merge.md and sessions-and-hooks.md. Worktree removal stays as it is: merge already detaches the work checkout and prints the `git worktree remove` command.

## Context

- `land --push` contract: docs/product/human-cli-output.md, Verdict line.
- Decision atelier-mzpn kept this standalone, outside atelier-mmpv, and dropped its receipt on the superseded mission atelier-ll3p.
- atelier-p1vz recounts piping on ordinary sessions; it measures, this item changes the product.
