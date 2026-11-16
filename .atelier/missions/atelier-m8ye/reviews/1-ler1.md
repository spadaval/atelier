---
schema: atelier.review
schema_version: 3
round: 1
commit: cf2cf2f32c0f2e03b7368cdb79c5c1d1b7a77db4
reviewers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A1f070e02-2027-406c-9056-2c2c6628de1d
model: claude-opus-5-5
verdict: changes
created_at: 2026-11-16T19:26:40.490288320Z
---

## Summary

I read the mission and its four items at cf2cf2f, and checked their claims against the code they name: `scripts/ci-jobs.sh`, `.github/workflows/ci.yml` and `release.yml`, `dist-workspace.toml`, `crates/atelier-desk/src/api.rs`, and `crates/atelier-core/src/forge.rs`, which merges with `gh pr merge --merge`, so an imported history survives the merge. I also read the extension's clone at `a242410`, including `install.py`, `config.example.json`, `test-extension.mjs`, and the objects its history holds. I checked with `gh` that `spadaval/t3-code-atelier` is private and `spadaval/atelier` is public. I also looked at recent CI timings: the `desk` job takes about 1m45s, so the six-minute budget for the extension job looks reachable with a Rust cache.

I could not run `dist plan` for an extension tag, because `dist` is not installed here. I also could not try a real T3 Code or Windows.

## Finding 1: The extension release must run the fixture test, but the mission says that release builds no Rust
Recommendation: fix now
atelier-451s's first Done when line says the tag workflow "runs the extension's fixture test as atelier-7tg6's job does". atelier-7tg6's job runs that test against "an `atelier-desk` built from the same commit", which means a cargo build of the Desk plus a pnpm build of `desk/`. The mission's Outcome says the extension's release "builds no Rust". An implementer can't meet both. The item should pick one:
- The tag workflow requires the CI run on the tagged `master` commit to have passed, and builds no Rust itself. This fits the mission and keeps the release fast.
- The mission allows the release to build `atelier-desk` for its test.

## Finding 2: The installer can't run on Linux, but three criteria need it to install into an empty home in CI
Recommendation: fix now
In `install.py` at `a242410` (lines 36 to 63), the installer stops with "T3 Code was not found" unless `appPath` exists. On any platform other than Windows, it then reads `<appPath>/Contents/Info.plist` from a macOS `.app`, even with `--home`. On a Linux runner with no T3 Code, it exits before writing `t3-custom`. Three criteria depend on that install working:
- The mission's own-release criterion and atelier-451s's no-publish run, which install "into an empty home directory".
- The mission's installs-from-bundle criterion, where `t3-custom check` passes in the installed home.
- atelier-wlyn's test of "an install from a bundle into an empty home", run by atelier-7tg6's job on Linux.

The plan doesn't say how any of these can pass. It should name a way. Two options: the test supplies a fake `appPath` holding an `Info.plist` and runs `t3-custom.mjs` with plain `node`, or the installer gains a mode for tests that skips finding T3 Code.

atelier-451s also asks for this install before atelier-wlyn changes the installer, so it hits the same wall one item earlier.

The plan also contradicts itself on macOS. atelier-wlyn says the installer works "on Windows and macOS as it does now", but the mission puts macOS "Out altogether". One of them should change.

## Finding 3: Importing the history publishes the private repository's screenshots of other projects
Recommendation: fix now
`spadaval/t3-code-atelier` is private and this repository is public. The history at `a242410` holds 15 PNGs under `artifacts/spike/`: `git rev-list --objects a242410 | grep png` lists them, though none is in the tree at that commit. One of them, `artifacts/spike/round2/threads-shown.png`, shows Sumeet's T3 Code sidebar, which includes:
- missions from his other repositories (astromech, demiurge, life-on-rails) with their titles;
- thread titles such as "Make SSH into Legend".

atelier-n4g7 asks for the history as it is. Once its branch is pushed, these images are public, and that can't be undone.

The item should either drop the blobs, or record that Sumeet accepts publishing them. Dropping them can be done with `git filter-repo --invert-paths --path artifacts` during the import. That still satisfies `git log --follow t3-extension/custom.js`. Either way, the item should name the check an implementer runs on the history before the first push.

## Finding 4: The extension job's rule leaves out its own workflow and the Desk's dependencies
Recommendation: fix now
atelier-7tg6 says the job runs "when a change touches `t3-extension/`, `desk/` or `crates/atelier-desk`, and not otherwise". Under that rule, two kinds of pull request skip the job that tests them:
- One that changes only the extension's job in `.github/workflows/ci.yml`, or its rule in `scripts/ci-jobs.sh`.
- One that changes only `Cargo.lock` or `Cargo.toml`, for example by bumping `serde` or the web framework the Desk serves through. That changes the Desk the extension talks to.

For comparison, `needs_rust` runs the `rust` job for any path it doesn't know, including `ci.yml`. The rule should add the workflow, `scripts/ci-jobs.sh`, and the workspace's `Cargo.toml` and `Cargo.lock`. The mission's quiet-for-others criterion still holds with these added.

## Finding 5: A change to atelier-core can change what the extension reads without running its test
Recommendation: no action
`SidebarResponse` and its fields are defined in `crates/atelier-desk/src/api.rs`, so renaming a field there does run the job. Its values come from `atelier-core`, though: mission state, the counts, and which items are waiting. Under quiet-for-others, a core change that alters them runs only the `rust` job. That trade-off seems reasonable for speed, and I'm noting it so it is made on purpose.
