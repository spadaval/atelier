---
schema: atelier.review
schema_version: 3
round: 3
commit: 0fbb46486de596cdb9a2b2ce82a095fc13b5dbb1
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A36efc1f0-18e7-4d78-90bd-89d9537c2a80
model: claude-opus-5-5
verdict: changes
created_at: 2026-11-16T19:32:41.514374945Z
---

## Summary

I read the mission and its four items at `0fbb464`, the diff from round 2 (`git diff 0241433 0fbb464`), both round-2 reviews, `scripts/ci-jobs.sh`, and `.github/workflows/ci.yml`. I checked the release item's tested-commit rule against how CI picks its base on `master`: the `push` run compares with `github.event.before`, and `master`'s first-parent history is one merge commit per push, so running `scripts/ci-jobs.sh C^1 C` on each first-parent commit reproduces what CI decided for it. The rule holds.

I did not read the extension's source, and I did not run anything. I could not confirm that a debug build of `atelier-desk` plus the Desk's app fits in six minutes; atelier-7tg6 measures that and reports it.

Two new problems, both small. A test atelier-wlyn adds cannot reach CI inside that item's scope. And atelier-451s asks for a hand-started run on `master` in its Report, which GitHub cannot run before the workflow is on `master`.

## Finding 1: The release refuses most `master` commits, because their CI run skipped the extension's job
Recommendation: fix now
Was: 2-soy4.8
Now: fixed
atelier-451s's first Done when line (`.atelier/items/atelier-451s/item.md:30`) now defines the tested commit. It is the newest commit on `master`'s first parent history, at or before the tag, that changes a path the extension's job runs for, judged by `scripts/ci-jobs.sh`. A tag on a later records-only merge is accepted on that commit's run. Line 33 tests both the accepted and the refused cases, and the mission's own-release criterion says the same.

## Finding 2: atelier-wlyn's tests must run in CI, but its scope holds only `t3-extension/` and the job runs one named script
Recommendation: fix now
atelier-7tg6's job runs `t3-extension/test-extension.mjs` (`.atelier/items/atelier-7tg6/item.md:29`). atelier-wlyn adds tests for release choice, the checksum refusal, an install into an empty home, and the auto-update setting, and says "The CI job from atelier-7tg6 runs them" (`.atelier/items/atelier-wlyn/item.md:34`). But atelier-wlyn's scope is `t3-extension/` alone (line 39), so it cannot edit `.github/workflows/ci.yml` to add them. Unless the job already runs every test in the folder, the new tests never run in CI, and the mission's installs-from-bundle criterion ("Tests cover each") is met only on the implementer's machine.

atelier-451s has the same gap for its test of the step that finds the tested commit (`.atelier/items/atelier-451s/item.md:33`): nothing says which CI job runs it.

Either fix works:
- atelier-7tg6's job runs one entry point that finds every test under `t3-extension/`, such as `node --test t3-extension/` or a `t3-extension/test.sh`, and the fixture test is one of them.
- atelier-wlyn's scope adds `.github/workflows/ci.yml`.

For atelier-451s, name where its test runs: for example, beside `scripts/ci-jobs.test.sh` in the `changes` job.

## Finding 3: The release item's Report must link a hand-started run on `master`, which GitHub can't start before the item merges
Recommendation: fix now
`.atelier/items/atelier-451s/item.md:34` asks for a workflow "started by hand with no tag, on `master`", and says "The Report links one such run". The mission's own-release criterion says the same. GitHub only starts a `workflow_dispatch` run for a workflow file that is already on the default branch. Before this item merges, its workflow is not on `master`, so the implementer can't start the run, and the Report is due before the merge.

Any one of these fixes it:
- The workflow also runs its build-and-install steps without publishing on a pull request that changes it or `t3-extension/`, and the Report links that run.
- The planner's thread starts the `master` run after the merge, and it moves to the mission's "done by the planner's thread" list.
- The implementer runs the workflow's steps on its branch some other way and says how in the Report. This is weaker, because it does not test the trigger.

## Finding 4: The first release's notes have no previous extension tag to count from
Recommendation: no action
`.atelier/items/atelier-451s/item.md:31` lists the items merged "since the previous extension tag". The first release has none, because atelier-n4g7 brings no tags over. The implementer will likely count from the import's merge. The item could say so, but this does not block the mission.
