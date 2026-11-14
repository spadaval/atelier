---
schema: atelier.review
schema_version: 3
round: 2
commit: 7227ed1ec3024b9508db0a5eb5b0a5949f8324d0
reviewers:
- thread:delegated-task:command%3Amcp%3Aba3b9e6f-8a49-4e3c-a239-c913be6c8ada%3Adelegate-task%3A24036feb-1d86-4100-b930-eb1d16d84c8d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T01:22:25.561472356Z
---


## Summary

Complexity: high. The forge now owns the final merge, and completion is derived across merge, planning, views and history rather than written into the item.
Value: high. Finishing through the pull request is necessary for Atelier to work on a protected base.
Risk of merging: medium. The merge is guarded and tested, but an incomplete reader fix still leaves fetched completion unrecognized in one supported catch-up path.
The value warrants the change. Keep finding 1 open for a diverged local base; finding 2 is fixed.

I reviewed commit 7227ed1ec3024b9508db0a5eb5b0a5949f8324d0 against master and the changes since round 1. I checked the forge merge and refusal paths, merged-tree checks, plan-review and record-only gates, hooks and skip, lock and moved-input checks, completion readers, history and design changes against the Done when lines. I ran 199 existing tests across merge, hooks, branches, deferred findings, disputes, scenarios, skip, planning, views, log and the app's view tests; all passed, with one other test skipped. This includes both new regression tests, the full protected-base loop and fresh-clone completion. `git diff --check master...7227ed1` passed. PR #159's test job was still running when inspected. I did not perform a live GitHub merge or change branch protection, and did not repeat the full workspace suite or clippy.

I also compiled a temporary copy of the merge integration harness outside the repository, with one independent diverged-base probe using its existing scratch repository and fake forge. That probe fails as described under finding 1. No implementation or repository test file was changed.

## Finding 1: Read the fetched forge base when the local base cannot fast-forward
Recommendation: fix now
Was: 1-buvp.1
Response: fixed in ac8880b62
Now: kept

The dirty-checkout case is fixed, but the diverged-base case from the original finding remains. `crates/atelier-app/src/repo.rs:65-69` reads the fetched forge base only if it contains the local base. When they diverge, it falls back to the local base, even though `crates/atelier-cli/src/merge.rs:411-413` explicitly permits a successful forge merge to leave that ref unchanged. Fetching the completed merge therefore still does not make Atelier recognize it in this path.

Reproduced with `Forge::new(None)`: check out `main` in a second worktree and commit an unrelated local-only file there after the item branch has forked. Run the approved item's merge. It exits 0, fetches the forge's merge and says the item is done, followed by the local-base warning. Immediately running `show <id>` still reads from the item branch and says `Status: approved`. Running merge again asks the fake forge to merge the pull request again instead of taking the already-merged path. Blockers likewise continue to see an approved item rather than a completed one. The new design paragraph documents the fallback but does not make those readers recognize a fetched, completed merge.

Use the fetched forge base to recognize completion independently of local-base divergence, while preserving and warning about the local commits. Extend the regression coverage to a diverged local base, asserting that show/status and a repeated merge recognize completion without moving that base or its checkout.

## Finding 2: Keep deferred findings visible on the item's merged branch
Recommendation: fix now
Was: 1-buvp.2
Response: fixed in ac8880b62
Now: fixed

`Repo::on_unmerged_branch_of` now distinguishes an unmerged own branch from one contained in the effective base, and both `locate_planned` and `views::deferred` use it. The new `a_merged_items_deferred_findings_are_listed_on_its_own_branch` test passes and verifies that the stored status stays approved while the deferred finding appears immediately after merge, with identical results on the base.
