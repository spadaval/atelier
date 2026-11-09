---
created_at: "2026-10-30T06:40:10.736546729+00:00"
id: "atelier-fad5"
issue_type: "feature"
labels:
- "forge"
workflow_branch:
  branch_base: master
  work_branch: atelier-fad5
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
closed_at: "2026-11-09T05:07:11.069928791+00:00"
status: "done"
title: "Let merge push the base it integrated, and stop recommending piped gates"
updated_at: "2026-11-09T05:07:11.069928791+00:00"
---

## Goal

Merge is the lifecycle's last step (atelier-jnrh), but agents still chain commands to publish it: a successful `atelier merge` ends `Next: atelier land, then git push.` (crates/atelier-cli/src/commands/merge.rs), and docs/product/human-cli-output.md (Verdict line) says `| tail -1` reads the verdict. In the 2026-10-15 to 10-29 transcripts, 276 of 399 `land`, `merge` and `check` runs were piped through `tail`, `head` or `grep`, and pushes, commits and branch deletions ran after refused gates; one such chain pushed unreviewed docs to master (incident 1620b534). Build the publishing half of `merge --push` as `docs/product/reviews-and-merge.md` ("`merge --push`, step by step", reviewed in atelier-ah82) describes it: steps 1, 4, 5, 6, 7 and 8, so integrating and publishing is one command whose exit status covers both. Pushing the branch, opening the pull request (steps 2 and 3) and `--wait` are atelier-vqoy's; until it lands, `--push` expects the branch already pushed with an open pull request, as today's hand-run routine leaves it.

## Done when

Integration tests with a bare repository as `forge.remote` and a fake `gh` show:
- [x] `merge <id> --push` on an approved branch whose CI passed fetches the base from `forge.remote`, fast-forwards a local base that is behind, merges, judges the unpushed base commits as `land` does, moves the remote base to the merge commit, and ends `merge: passed — pushed <base> to <remote>, pull request #<n> merged`, exit 0. `merge --push` with no ID does the same on an approved light-path branch.
- [x] With a refused gate, or `gh` missing or signed out (refused before anything is pushed), the remote base is unchanged, the verdict says nothing was pushed, and the exit status is non-zero.
- [x] A local base that has diverged from the remote base refuses with `merge_push_base_diverged`, naming both heads, before merging; when the integration succeeds but the base push is rejected, it refuses with `merge_push_base_rejected`, the merge commit stays on the local base, the exit status is non-zero, and the Next is the documented `git fetch <remote> <base> && git merge --no-edit FETCH_HEAD`, `atelier land`, `git push <remote> <base>`, not `land --push`.
- [x] When the fake `gh` does not report the pull request merged within the wait, `merge` exits 0 and prints the documented "still open" note, closing nothing; the wait is short and injectable in tests.
- [x] A policy without a forge refuses `merge --push` with `merge_push_no_forge` and pushes nothing; plain `merge` there behaves as before except that its success Next names `atelier land --push` instead of a chained `git push`, and in a forge policy names `atelier merge <id> --push` for next time.
- [x] No file under docs/product recommends piping a gate's output to read its verdict (`git grep -n "tail -1" docs/product` shows no such advice), and the docs say which `--push` steps exist until atelier-vqoy lands.

## Scope

crates/atelier-cli/src/commands/merge.rs, the base judgment and push code it reuses from `land --push` (commands/session.rs), the forge provider in crates/atelier-app/src/forge.rs (pull request state read; no branch push or pull request creation), and docs/product/human-cli-output.md, reviews-and-merge.md and sessions-and-hooks.md. Not: steps 2 and 3, `--wait`, `--timeout` (atelier-vqoy). Worktree removal stays as it is: merge already detaches the work checkout and prints the `git worktree remove` command.

## Context

- The interface is fixed by docs/product/reviews-and-merge.md ("CI on a forge" and "`merge --push`, step by step") and ADR 0031; atelier-7tul built the provider and `ci.passed` (crates/atelier-app/src/forge.rs, ci_passed.rs). Where building shows the docs wrong, fix them on the branch and say so in the Report.
- `land --push` contract: docs/product/human-cli-output.md, Verdict line. `land --push` pushes to the base's upstream; `merge --push` pushes to `forge.remote`, which may be a URL.
- Decision atelier-mzpn kept this standalone, outside atelier-mmpv, and dropped its receipt on the superseded mission atelier-ll3p.
- atelier-p1vz recounts piping on ordinary sessions; it measures, this item changes the product.

## Notes

- 2026-11-08 [handoff] Implementation handoff: code at ebd76dd6, Report at 33220a6d. All Done when lines ticked with named tests in forge_ci.rs; full atelier-cli suite passes with and without test-hooks; smoke-repos no regression. Review outstanding. See ## Report for decisions, doc fixes and notes for atelier-vqoy.

## Report

All six Done when lines hold, each shown by a test in crates/atelier-cli/tests/forge_ci.rs (bare repository behind a fake ssh as `forge.remote`, fake `gh` on PATH), passing with and without `--features test-hooks` except where noted: (1) `merge_push_publishes_the_merge_and_confirms_the_pull_request_merged` (fetch, fast-forward of a behind local base, merge, base judgment, remote base at the merge commit, `merge: passed — pushed main to origin, pull request #7 merged`) and `merge_push_publishes_an_approved_light_path_branch_to_a_url_remote` (no ID, light path, `forge.remote` written as a URL); (2) `merge_push_pushes_nothing_when_a_gate_refuses_or_gh_is_missing_or_signed_out`; (3) `merge_push_refuses_a_local_base_that_diverged_from_the_forge_before_merging` and `merge_push_keeps_the_merge_when_the_forge_rejects_the_base_push` (the fake `gh` moves the forge's base during the CI lookup); (4) `merge_push_passes_with_a_note_when_the_pull_request_is_still_open` (test-hooks only, see Decisions); (5) `merge_push_needs_a_forge_and_plain_merge_names_land_push` and `plain_merge_in_a_forge_policy_names_merge_push_for_next_time`, plus the updated Next in merge_gates.rs; (6) `git grep -n "tail -1" docs/product` prints nothing, and reviews-and-merge.md says `--push` runs steps 1 and 4 to 8 until atelier-vqoy. Also `merge_push_judges_the_unpushed_base_commits_as_land_does` (an unreviewed code commit on the local base refuses with `land_base_unreviewed` after the merge, nothing pushed). Full atelier-cli suite: 851 passed, 858 with test-hooks. `scripts/smoke-repos.sh --new <branch build>`: no regression across 5 repositories.

### Decisions

- The flow lives in a new module, crates/atelier-cli/src/commands/merge_push.rs, which calls merge's own `attempts` for steps 5 and 6 (now returning what it integrated) and land's base judgment for step 7. That judgment was split: `check_base_commits` resolves the upstream and calls the new `check_base_commits_since`, which `merge --push` calls with the base it fetched, since `forge.remote` need not be the base's upstream.
- The pull request is the open one from the branch into the base (`gh api repos/<slug>/pulls -f head=<owner>:<branch> -f base=<base> -f state=open`), read after the merge and before the push, because the branch head is often a local approval GitHub has never seen (finding atelier-sihf). No pull request and a failed lookup each print their own note; the verdict then names only the push.
- Review fixes: a separately checked-out base is fast-forwarded only after merge's `CommittedSnapshot::require_clean` and untracked-overwrite checks pass (atelier-kl16); the forge is re-read from the fetched base before merging and from the merge's first parent before pushing, refusing `merge_push_no_forge` or `merge_push_forge_changed` (atelier-gfwz). The post-merge re-check cannot be reached by a test today, since merge retries only on record-only base moves and the policy is code. Tests: `merge_push_refuses_to_fast_forward_a_base_checkout_with_an_uncommitted_record`, `merge_push_reads_the_forge_from_the_fetched_base`, `merge_push_confirms_the_pull_request_when_the_approval_was_never_pushed`, `merge_push_says_whether_it_found_no_pull_request_or_could_not_look`.
- The merged wait is 60 seconds, polling `repos/<slug>/pulls/<n>` every 5 seconds. Tests set it with `ATELIER_TEST_MERGE_PR_WAIT_SECS`, read only under the `test-hooks` feature, so production has no hidden knob; the still-open test is therefore test-hooks only (CI runs that suite).
- Step 1 runs `gh auth status --hostname <host>`; `ForgeError`'s existing detection now covers it through one `gh` runner in forge.rs.
- On the base or a detached HEAD, `--push` lets merge refuse (or report already merged, ending `merge: passed, nothing pushed`) without fetching.
- Tests reach `git@github.com:owner/repo.git` through `GIT_SSH_COMMAND` pointing at a fake ssh that serves the bare repository, because `Github::open` must still find a GitHub slug in the remote's URL; a local path as `forge.remote` names no GitHub repository.

### Deviations

- Docs fixed on the branch: reviews-and-merge.md now names `merge_push_fetch_failed` (step 4 had no code for a failed fetch), says a local base ahead of the forge's is kept and judged in step 7, says step 7 judges from the fetched base and prints land's summary line, gives the poll interval, and says ci.passed's hand-run Next lines omit `--push` until atelier-vqoy. human-cli-output.md replaces the `| tail -1` advice with "act on the exit status", adds merge --push verdicts, and gives plain merge's new Next; sessions-and-hooks.md points publishing at `land --push` / `merge --push`.

### Open items

- For atelier-vqoy: ci.passed's Next lines are built in atelier-app (workflow_validation.rs) as `atelier merge <id>` and do not keep `--push`; merge's own reruns do (`merge::rerun(issue_ref, push)`). The pull request lookup in step 8 should switch to the one step 3 ensures.
- docs/product/cli-surface.md line 19 still describes the full `--push` (branch push and pull request); left as the target interface.
- The `ForgeError` reasons still say "so Atelier cannot read CI", also for `merge --push`'s step 1.
