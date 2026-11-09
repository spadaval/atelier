---
created_at: "2026-11-09T02:02:35.262010794+00:00"
id: "atelier-vqoy"
issue_type: "feature"
labels:
- "forge"
workflow_branch:
  branch_base: master
  work_branch: atelier-vqoy
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-7tul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fad5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T07:54:04.436722143+00:00"
status: "done"
title: "Let merge push the work branch, open its pull request and wait for CI"
updated_at: "2026-11-09T07:54:04.436722143+00:00"
---

## Goal

Build the rest of `merge --push` and `--wait` as `docs/product/reviews-and-merge.md` describes them ("`merge --push`, step by step", steps 2 and 3, and "Waiting for CI with `--wait`"): `--push` pushes the work branch to `forge.remote` and makes sure it has a pull request into the base before the gates, and `--wait` polls CI until it finishes or a timeout passes, so no agent runs `git push` or `gh pr create` by hand. With atelier-fad5's publishing half, approved work reaches the published base in one command.

## Done when

Integration tests with a bare repository as `forge.remote` and a fake `gh` show:
- [x] `merge <id> --push` on an approved branch with no pull request pushes the branch (never forcing), opens a pull request titled `<id>: <title>` and prints `Pull request #<n>: <url>` saying it opened it, then refuses with `ci_pending` naming the run and its link while CI is pending; a rerun reuses the pull request, and once CI passes merges and publishes. A light-path branch's pull request takes its newest commit subject as the title.
- [x] Steps 2 and 3 run even when a later gate refuses, such as a missing approval; a rejected branch push refuses with `merge_push_branch_rejected` and the documented `git fetch <remote> <branch> && git merge --no-edit FETCH_HEAD` Next, and pushes no base.
- [x] `merge <id> --push --wait` with CI pending then passing merges and publishes in one run, printing a line when CI's state changes; with CI failing it refuses with `ci_failed` naming the run; past `--timeout` it refuses with `ci_pending` saying how long it waited, its Next keeping `--push`; a run not started within its grace period refuses with `ci_no_run`. Poll interval and grace period are injectable in tests. `--wait` without `--push` waits, then merges without pushing.
- [x] `--push`, `--wait` and `--timeout` refuse in combination with `--check`, `--json` or `--sync`, and `--timeout` without `--wait`.
- [x] `ci.passed`'s Next lines name `atelier merge <id> --push --wait` and `[--push] --wait` as the refusal table documents, the interim hand-step wording is removed from the docs and the code, and `merge`'s `--help` documents `--push`, `--wait` and `--timeout`.

## Scope

crates/atelier-cli/src/commands/merge.rs and main.rs (flags and help), crates/atelier-app/src/forge.rs and ci_passed.rs, and docs/product/reviews-and-merge.md. AGENTS.md, the skill and `scripts/merge-ci-passed.sh` are atelier-7o4r's.

## Unknowns

- How long GitHub takes to show an indirect merge after the push, and so how long `merge` should look before saying the pull request is still open. The real observation is atelier-7o4r's.
- The `--wait` default timeout (documented as 30 minutes), given Claude Code's 10-minute foreground command limit and this repository's 12-minute macOS run. Agents in such harnesses run it in the background or pass `--timeout`; atelier-7o4r's real run shows whether the default needs to change.

## Report

All five Done when lines hold, shown in `crates/atelier-cli/tests/forge_ci.rs` (33 tests, all passing with and without `test-hooks`; the four that poll are `test-hooks` only): `merge_push_pushes_the_branch_and_opens_its_pull_request_then_publishes_once_ci_passes` (push without forcing, upstream set, pull request `<id>: <title>` opened and printed `Pull request #7: <url> (opened now)`, pending CI refused with the run's link and `Next: atelier merge <id> --push --wait`, rerun reuses it and publishes) and `merge_push_titles_a_light_path_pull_request_by_its_newest_code_commit`; `merge_push_pushes_the_branch_and_opens_its_pull_request_when_a_gate_refuses` (stale approval), `merge_push_pushes_nothing_without_gh_and_no_base_when_a_gate_refuses` and `merge_push_refuses_a_rejected_branch_push_and_pushes_no_base`; `merge_push_wait_merges_and_publishes_once_pending_ci_passes`, `merge_wait_refuses_on_a_failed_run_or_at_its_timeout`, `merge_push_wait_refuses_a_run_that_never_starts_after_its_grace_period` and `merge_wait_without_push_waits_then_merges_without_pushing` (poll and grace injected by `ATELIER_TEST_MERGE_CI_POLL_MS` and `ATELIER_TEST_MERGE_CI_GRACE_MS`); `push_wait_and_timeout_refuse_check_json_sync_and_a_timeout_without_wait`; and the updated `ci_passed_refuses_with_the_runs_state_and_link_or_when_no_run_exists` for the new Next lines. `atelier merge --help` documents `--push`, `--wait` and `--timeout <MINUTES>`.

### Deviations

- Light-path title: the title is the subject of the branch's newest commit that changes code, falling back to the newest commit. Taken literally, "newest commit subject" on an approved light branch is the approval's record commit. Docs say so; the Done when line was not rewritten, so the reviewer should judge whether this reading holds.
- The refusal codes `ci_pending`, `ci_failed` and `ci_no_run` are the refusal table's names; the output is the `ci.passed` gate line with its reason and Next, as before. `CiState` in `ci_passed.rs` carries them for `--wait`.
- Docs fixed where building showed them wrong: step 5 said a not-yet-started run "reads as pending" right after a push; it refuses with `ci_no_run`, whose Next (`--push --wait`) is the same command `ci_pending` gives, and `--wait` counts it pending for the grace period. Refusal verdicts said `nothing pushed` even after the branch push; they now say `master not pushed` once the branch is on the forge (also in `human-cli-output.md`), and the later step refusals say `<base> was not pushed`. Step 8's notes for "no open pull request" and "could not read" are gone, since step 8 uses the pull request step 3 found or opened; a failed lookup or create in step 3 refuses with `forge_unreachable`.
- Added `merge_push_branch_failed` for a branch push that fails for a reason other than `[rejected]` (credentials, network), so it is not told to merge the remote branch (`merge_push_names_a_failed_branch_push_apart_from_a_rejected_one`). Documented in step 2.

### Open items

- Step 7's base push still calls any failure `merge_push_base_rejected` (atelier-fad5's code); an auth failure would now stop at step 2 first.
- The real GitHub run is atelier-7o4r's: this repository's `forge.remote` is `origin`, which here is HTTPS without credentials, so step 2 would refuse with `merge_push_branch_failed` until `origin` gets an SSH push URL (`git remote set-url --push origin git@github.com:spadaval/atelier.git`) or the policy names an SSH URL.
- `--wait` refuses on the first `gh` error while polling rather than retrying.
