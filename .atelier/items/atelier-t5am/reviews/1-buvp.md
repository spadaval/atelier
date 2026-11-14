---
schema: atelier.review
schema_version: 3
round: 1
commit: 14036de53028e00569d2be0f9ee4b20a96095b24
reviewers:
- thread:delegated-task:command%3Amcp%3Aba3b9e6f-8a49-4e3c-a239-c913be6c8ada%3Adelegate-task%3Aa443436b-9444-432f-a727-270c3025a68c
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-14T01:15:03.136288396Z
---

## Summary

Complexity: high. The forge now owns the final merge, and `done` becomes a reader contract shared by merge, planning, views and history.
Value: high. Finishing through the pull request is necessary for Atelier to work on a protected base.
Risk of merging: medium. The merge path is guarded and tested, but the reader changes can hide completed work or its deferred findings.
The value warrants the change; fix the two reader regressions below before merging.

I reviewed commit 14036de53028e00569d2be0f9ee4b20a96095b24 against master, including the forge calls, merged-tree checks, plan-review and record-only gates, hooks, lock and moved-input checks, status readers, history and design changes. I ran 197 existing tests covering merge, hooks, branches, deferred findings, disputes, scenarios, skips, planning, views and log; all passed (one additional test was skipped). `git diff --check master...14036de` passed. PR #159's test job passed. The protected-base contract test covers the full item loop and fresh-clone status. I did not perform a live GitHub merge or change branch protection.

I also ran two independent regression probes in temporary files, using this change's existing scratch-repository and fake-forge helpers. Both failed as described below. No implementation or repository test file was changed.

## Finding 1: Read the fetched forge base when the local base cannot fast-forward
Recommendation: fix now

`crates/atelier-cli/src/merge.rs:416-423` permits a successful forge merge to leave the local base behind when its checkout has changes or its fast-forward fails. However, `Repo::opened` still chooses only `refs/heads/<base>` (`crates/atelier-app/src/repo.rs:174`), so fetching the successful merge does not make Atelier recognize it. The same problem occurs with a diverged local base. This is the reader rule the Report acknowledges is unimplemented, but the new success path now depends on it.

Reproduced with `Forge::new(None)`: check out `main` in a second worktree, append an uncommitted comment to its config, and run `merge` from the approved item branch. Merge exits 0, says the item is done, and fetches the merge into `origin/main`. Immediately running `show <id>` still says `Status: approved`, read from the item branch. Running merge again does not take the already-merged path; in the contract double it requests another pull request and merges again. A real forge can instead refuse to create that redundant pull request because there are no commits to merge. Blockers and mission completion also continue to see the old status until the local base moves.

Use the configured forge's remote-tracking base as the reader's base when it exists, as the design specifies, independently of whether its local checkout can advance. Preserve the user's uncommitted work. Add coverage asserting that show/status and a repeated merge recognize completion when catch-up leaves the local base unchanged.

## Finding 2: Keep deferred findings visible on the item's merged branch
Recommendation: fix now

`crates/atelier-cli/src/views.rs:273-283` skips `on_base` whenever the checkout is the item's own branch, even when that branch has already merged. Since this change leaves the stored status `approved` (or `building` for a trivial merge), `unsettled` then returns no findings: it requires `Status::Done`. Merge leaves the checkout on this branch, making this the normal place to run the next command.

Reproduced without a forge: submit an item, approve it with one `Recommendation: defer` finding, merge, and run `list --deferred` without switching branches. `show <id>` correctly says done, but the list says `No deferred finding of a done item is left to settle.` Checking out the merged base immediately makes the same finding appear. The probe comparing those two lists fails.

Distinguish an unmerged own branch from a merged own branch when deriving status, and test a newly merged item whose stored status remains approved. Deferred findings must remain visible after merge without requiring a branch switch.
