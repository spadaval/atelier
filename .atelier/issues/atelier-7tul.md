---
created_at: "2026-11-09T02:02:16.894255456+00:00"
id: "atelier-7tul"
issue_type: "feature"
labels:
- "forge"
workflow_branch:
  branch_base: master
  work_branch: atelier-7tul
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
    id: "atelier-ah82"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add the GitHub forge and a ci.passed merge check, so a docs-only base change no longer forces a CI rerun"
updated_at: "2026-11-09T03:29:57.326306212+00:00"
---

## Goal

Build what atelier-ah82 documents: a GitHub forge provider that shells out to `gh`, and a typed `ci.passed` merge check that passes when the trusted CI workflow succeeded on a pull request run for a commit whose code matches the branch head. Switch this repository's policy from `scripts/merge-ci-passed.sh` to it, so once this merges a base that gained only docs, strategies or records since the CI run no longer needs a sync and a new run. Keep the script: this branch merges under master's committed policy, which still runs it, and atelier-7o4r deletes it once master's policy uses `ci.passed`.

## Done when

- [x] Integration tests with a fake `gh`: `ci.passed` passes when CI succeeded on the branch head, or on an earlier branch commit with the same code; refuses with the run's state and link when it is pending or failed, and when no run exists; and refuses naming `gh auth login` when `gh` is missing or signed out.
- [x] A test where the base gained a docs-only commit after the branch's CI run: `ci.passed` passes without a new run, and the merge's local `shell.check`s still run on the merged tree.
- [x] A test where the base gained a code commit after the run: `ci.passed` still passes, and the merge output says CI on the base after the push covers the combination, as atelier-ah82's docs say.
- [x] A policy with no forge parses and merges exactly as before (existing merge tests pass unchanged), and `atelier check` refuses a policy whose forge or CI workflow key is malformed, naming the key.
- [x] `.atelier/workflow.yaml` uses `ci.passed` in place of the script's `shell.check`, and the script stays. An integration test shows the switch: with a base policy that runs a CI script check and a branch whose policy switches to `ci.passed`, the branch's merge runs the base's check, and the next branch's merge onto the new base runs `ci.passed`.

## Scope

The provider, `ci.passed`, policy parsing and this repository's policy. Not pushing branches or the base, pull requests, or `--wait` (atelier-fad5 and the next item).

## Context

The script's logic (code paths, matching an earlier commit's code, the record-only shortcut) is in `scripts/merge-ci-passed.sh`; keep its record-only behaviour. Merge checks run from the base's committed policy, so this branch merges under the script's check, which stays. The installed binary cannot read the branch's `ci.passed` policy and refuses the merge with `workflow_config_invalid_validator` (reproduced in planning review finding atelier-8wdb), so run this item's merge with the reviewed branch build: `cargo install --path crates/atelier-cli --locked` from the approved head, or `target/debug/atelier merge <id>` built from it, then `atelier land` with that binary. The base policy and the script are unchanged by doing so.

## Unknowns

- Whether `gh run list --commit` or `gh api` gives stable enough output; the strategy's Risks section says what to do if not.

## Notes

- 2026-11-08 [decision] Estimate about 800 lines: forge block and ci.passed parsing in atelier-workflow, a gh provider and the ci.passed check in atelier-app, merge output and commit message in atelier-cli, a forge_ci integration test with a fake gh on PATH, and this repo's policy.
  Reason: The implement procedure asks for a size estimate before editing.
- 2026-11-08 [decision] No schema_version bump for the forge block.
  Reason: The block is optional and additive, so a policy without it parses exactly as before. An older atelier cannot misread a forge policy: it already refuses the unknown top-level key forge (workflow_config_unknown_field) and the unknown validator ci.passed, which is the failure a version bump guards against. A bump would make every new binary accept versions 3 and 4 and tie the forge key to 4, only to change the old binary's message to 'needs a newer atelier'. This repo's switch is handled by merging with the branch build and reinstalling, as the item's Context says.
- 2026-11-08 [note] Handoff: implementation committed at 52d90f75; see ## Report. Review outstanding (not requested, per the assignment). Merge with the branch build, then reinstall before land.

## Report

All five Done when lines hold, proven by `crates/atelier-cli/tests/forge_ci.rs` (8 tests, all pass) against a fake `gh` on `PATH`: `ci_passed_passes_on_the_head_or_an_earlier_commit_with_its_code` (run on the head, then only on the implementation commit before two record-only commits; the pass line is printed and repeated in the merge commit message); `ci_passed_refuses_with_the_runs_state_and_link_or_when_no_run_exists` (in progress, queued, failed, cancelled, newest run decides, no pull request, pull request without a run, and a passing run behind a later code change is not used); `ci_passed_names_gh_auth_login_when_gh_is_missing_or_signed_out`; `a_branch_that_changes_only_records_needs_no_run_and_no_gh`; `a_docs_only_base_change_after_the_run_needs_no_new_run_and_shell_checks_run_on_the_merged_tree` (a shell check that only the merged tree passes); `a_code_base_change_after_the_run_still_passes_and_says_base_ci_covers_the_combination` (also shows the failed-base-CI note); `atelier_check_refuses_a_malformed_forge_naming_the_key` (provider, ci_workflow, remote, unknown key, ci.passed without forge); `switching_the_policy_to_ci_passed_takes_effect_on_the_next_merge` (the switch merges under the base's script check with no gh call; the next branch merges under ci.passed). Existing merge tests pass unchanged: `cargo nextest run -p atelier-cli` 841/841 and with `--features test-hooks` 847/847; atelier-app and atelier-workflow 150/150, including a new parse test. Disabling the walk stop and the base-gained-code test together made the two tests that cover them fail. `.atelier/workflow.yaml` now names the forge and runs `ci.passed` in place of `scripts/merge-ci-passed.sh`, which stays; the branch build's `atelier check` passes on it.

### Deviations

- The Next lines for `ci_no_run`, `ci_pending` and `ci_failed` name manual steps (`gh pr create --base <base>`, `gh run watch <run-id>`, push, then `atelier merge <id>`) instead of `atelier merge <id> --push --wait`, since those flags are out of scope until atelier-vqoy. `docs/product/reviews-and-merge.md` now says so under the refusal table.
- The failed-base-CI note reads the trusted workflow's newest `push` run on the base and shows it when that run finished and failed; the doc said "newest completed run for the remote base's head", which needs a second call to learn that head. The doc now says what is built. A forge that cannot answer adds no note.
- `forge.ci_workflow` must be a file name, not a path; the doc now says so.
- AGENTS.md's Merge Policy named `scripts/merge-ci-passed.sh` as the check; it now names `ci.passed`, since this branch switches the policy.
- Size: about 1,440 lines (about 540 of them the new test file), past the 800 estimate but under twice it.

### Decisions

See the notes: no `schema_version` bump. The provider calls the REST API through `gh api` with fixed fields (`actions/workflows/<file>/runs?head_sha=&event=pull_request`, `commits/<sha>/pulls`, and `runs?branch=&event=push` for the base note) rather than parsing `gh run list`, per the strategy's Risks; exit 4 or a `gh auth login` hint reads as signed out.

### Surprises

- GitHub reports no pull request number on a run once its pull request is closed (`pull_requests: []` for 6510969f's run), so a pass line then says "a pull request" instead of "pull request #N".
- The pass line for a base that gained docs only says the base gained code, since docs count as code under the script's and the docs' definition; it passes either way.

### Open items

- Merge this item with the branch build (`$CARGO_TARGET_DIR/debug/atelier merge atelier-7tul` from the approved head), since the installed binary refuses the branch's policy, then `cargo install --path crates/atelier-cli --locked` before `atelier land` and push, as the Context says. The base's script check still runs for this merge and needs CI green on the pull request.
- atelier-7o4r deletes `scripts/merge-ci-passed.sh` once master runs `ci.passed`.
- `ci.passed` in a session `land` on a work branch whose owner is already done (an older-Atelier state) reads HEAD and the local base branch instead of a merge commit; untested beyond the merge path.
