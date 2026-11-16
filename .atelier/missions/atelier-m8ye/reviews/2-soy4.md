---
schema: atelier.review
schema_version: 3
round: 2
commit: a07e9d09344a52a7a67b2428eff587a1c5cb3f24
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3Aba3642b4-9c6f-4888-afd6-5265e1ae9f63
model: claude-opus-5-5
verdict: changes
created_at: 2026-11-16T19:30:39.127492192Z
---

## Summary

I read the revised mission and its four items at `a07e9d0`, and the diff from `0241433` that answers round 1. I checked the answers against `.github/workflows/ci.yml` (its triggers and how the `changes` job picks a base), `scripts/ci-jobs.sh`, and how `master` merges land (merge commits with two parents, as `883f39f7f` shows, so imported history survives the merge). I did not reread `install.py` or the private repository's history; for findings 2 and 3 I relied on round 1's evidence and checked only that the plan now answers it. I could not check anything on Windows, macOS or a real T3 Code.

All seven earlier findings are fixed. One new problem remains, in how the release workflow decides that a tagged commit was tested (Finding 8).

## Finding 1: The extension release must run the fixture test, but the mission says that release builds no Rust
Recommendation: fix now
Was: 1-ler1.1
Now: fixed
atelier-451s now builds no Rust and runs no test. Instead it checks that the tagged commit's CI run passed with the extension's job in it. The mission's Outcome says the same. The contradiction is gone, though the check has a gap of its own (Finding 8).

## Finding 2: The installer can't run on Linux, but three criteria need it to install into an empty home in CI
Recommendation: fix now
Was: 1-ler1.2
Now: fixed
atelier-451s now asks the installer for a documented option that names a stand-in for T3 Code's app. atelier-wlyn's tests use that option and are blocked by atelier-451s, so the order holds. The mission's installs-from-bundle criterion names the stand-in. The macOS contradiction is settled: the macOS paths stay, and nothing proves them.

## Finding 3: Importing the history publishes the private repository's screenshots of other projects
Recommendation: fix now
Was: 1-ler1.3
Now: fixed
atelier-n4g7 now rewrites the history in a scratch clone so that no commit holds `artifacts/`, and scans every commit before the push. A finding other than the authors' Git identities stops the push. The mission adds a nothing-private criterion, and its Authority section makes such a push a question for the starting thread.

## Finding 4: The extension job's rule leaves out its own workflow and the Desk's dependencies
Recommendation: fix now
Was: 1-ler1.4
Now: fixed
The rule in atelier-7tg6 now adds `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `ci.yml` and `scripts/ci-jobs.sh`. It also leaves out `crates/atelier-core` on purpose, says why, and has the README say so.

## Finding 5: Allow the documentation changes in the import's tree comparison
Recommendation: fix now
Was: 1-s4iu.1
Now: fixed
The comparison is now byte for byte at the last imported commit, with every change made here in later commits. That lets the README edits happen without breaking same-tree. Merges here keep both parents, so that commit stays in `master`'s history.

## Finding 6: Decide whether extension releases may build the Desk for testing
Recommendation: fix now
Was: 1-s4iu.2
Now: fixed
Settled the same way as Finding 1: the release builds no Rust and relies on the CI run of the tagged commit.

## Finding 7: Make the mission's exclusions match the updater item
Recommendation: fix now
Was: 1-s4iu.3
Now: fixed
The mission now carves the installed-version display and update control out of the behaviour exclusion. It replaces "macOS, out altogether" with "nothing on macOS is checked; its paths stay", and atelier-wlyn says the same.

## Finding 8: The release refuses most `master` commits, because their CI run skipped the extension's job
Recommendation: fix now
atelier-451s's first Done when line (`.atelier/items/atelier-451s/item.md:30`) and the mission's own-release criterion require "the CI run on the tagged commit" to have passed with the extension's job run and not skipped. On `master`, that run is the `push` run, and `.github/workflows/ci.yml` sets the `changes` job's base to `github.event.before`. So the extension's job runs on a `master` commit only when that commit's own merge touched one of atelier-7tg6's paths. For any later merge, such as a records-only `backlog/` merge or a change to `atelier-core`, the job is skipped, and the workflow must refuse the tag even though the extension's files are unchanged and were tested.

The planner tags the first release after the items merge. If any other merge lands first, the tag on `master`'s tip is refused, and the README's "one command" fails without saying why. The item should state the rule. Either of these would do:
- Tags go on the merge commit that last changed the extension's paths. The README's command finds that commit, and the refusal message names it.
- The workflow accepts a tag when the newest earlier `master` commit that touched those paths passed with the extension's job, and no later commit up to the tag touched them.
