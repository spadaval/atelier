---
created_at: "2026-10-30T06:40:10.736546729+00:00"
id: "atelier-fad5"
issue_type: "feature"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let merge push the base it integrated, and stop recommending piped gates"
updated_at: "2026-11-09T04:11:33.511430640+00:00"
---

## Goal

Merge is the lifecycle's last step (atelier-jnrh), but agents still chain commands to publish it: a successful `atelier merge` ends `Next: atelier land, then git push.` (crates/atelier-cli/src/commands/merge.rs), and docs/product/human-cli-output.md (Verdict line) says `| tail -1` reads the verdict. In the 2026-10-15 to 10-29 transcripts, 276 of 399 `land`, `merge` and `check` runs were piped through `tail`, `head` or `grep`, and pushes, commits and branch deletions ran after refused gates; one such chain pushed unreviewed docs to master (incident 1620b534). Build the publishing half of `merge --push` as `docs/product/reviews-and-merge.md` ("`merge --push`, step by step", reviewed in atelier-ah82) describes it: steps 1, 4, 5, 6, 7 and 8, so integrating and publishing is one command whose exit status covers both. Pushing the branch, opening the pull request (steps 2 and 3) and `--wait` are atelier-vqoy's; until it lands, `--push` expects the branch already pushed with an open pull request, as today's hand-run routine leaves it.

## Done when

Integration tests with a bare repository as `forge.remote` and a fake `gh` show:
- [ ] `merge <id> --push` on an approved branch whose CI passed fetches the base from `forge.remote`, fast-forwards a local base that is behind, merges, judges the unpushed base commits as `land` does, moves the remote base to the merge commit, and ends `merge: passed — pushed <base> to <remote>, pull request #<n> merged`, exit 0. `merge --push` with no ID does the same on an approved light-path branch.
- [ ] With a refused gate, or `gh` missing or signed out (refused before anything is pushed), the remote base is unchanged, the verdict says nothing was pushed, and the exit status is non-zero.
- [ ] A local base that has diverged from the remote base refuses with `merge_push_base_diverged`, naming both heads, before merging; when the integration succeeds but the base push is rejected, it refuses with `merge_push_base_rejected`, the merge commit stays on the local base, the exit status is non-zero, and the Next is the documented `git fetch <remote> <base> && git merge --no-edit FETCH_HEAD`, `atelier land`, `git push <remote> <base>`, not `land --push`.
- [ ] When the fake `gh` does not report the pull request merged within the wait, `merge` exits 0 and prints the documented "still open" note, closing nothing; the wait is short and injectable in tests.
- [ ] A policy without a forge refuses `merge --push` with `merge_push_no_forge` and pushes nothing; plain `merge` there behaves as before except that its success Next names `atelier land --push` instead of a chained `git push`, and in a forge policy names `atelier merge <id> --push` for next time.
- [ ] No file under docs/product recommends piping a gate's output to read its verdict (`git grep -n "tail -1" docs/product` shows no such advice), and the docs say which `--push` steps exist until atelier-vqoy lands.

## Scope

crates/atelier-cli/src/commands/merge.rs, the base judgment and push code it reuses from `land --push` (commands/session.rs), the forge provider in crates/atelier-app/src/forge.rs (pull request state read; no branch push or pull request creation), and docs/product/human-cli-output.md, reviews-and-merge.md and sessions-and-hooks.md. Not: steps 2 and 3, `--wait`, `--timeout` (atelier-vqoy). Worktree removal stays as it is: merge already detaches the work checkout and prints the `git worktree remove` command.

## Context

- The interface is fixed by docs/product/reviews-and-merge.md ("CI on a forge" and "`merge --push`, step by step") and ADR 0031; atelier-7tul built the provider and `ci.passed` (crates/atelier-app/src/forge.rs, ci_passed.rs). Where building shows the docs wrong, fix them on the branch and say so in the Report.
- `land --push` contract: docs/product/human-cli-output.md, Verdict line. `land --push` pushes to the base's upstream; `merge --push` pushes to `forge.remote`, which may be a URL.
- Decision atelier-mzpn kept this standalone, outside atelier-mmpv, and dropped its receipt on the superseded mission atelier-ll3p.
- atelier-p1vz recounts piping on ordinary sessions; it measures, this item changes the product.
