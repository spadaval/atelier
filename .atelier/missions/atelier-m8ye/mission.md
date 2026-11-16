---
schema: atelier.mission
schema_version: 3
id: atelier-m8ye
title: Bring the T3 Code extension into this repository, test it in CI, and release it on its own tags
status: open
labels: []
lead: null
planners:
- c6689ff2-b6ae-4865-afe4-a3533c8cfa80
created_at: 2026-11-16T19:25:23.866165678Z
---

## Why

The extension that shows the Desk inside T3 Code lives in its own
repository, `github.com/spadaval/t3-code-atelier`, and every change to it in
the three days it has existed also changed the Desk. Missions atelier-61gh
and item atelier-cdxg changed the Desk's routes and its message contract in
step with the extension, and the planner's thread kept the two in step by
hand: field names fixed in briefs, a second Desk on port 3781, and merges in
a set order. The extension's reviews were asked for by hand, and its fixture
test runs against whichever Desk is live on buildbot, so the two can drift
without a test failing.

Sumeet asked on 2026-11-16, in conversation, whether the extension should be
folded into this repository, and the same day directed this mission: "do
this, and the rest of the mission". He set two conditions in that
conversation. The extension gets a CI test suite. And it is released more
often than `atelier`, on its own tags, as a bundle a device downloads, since
a device should not need a clone of a Rust project to run five script files.

## Outcome

The extension's code is in this repository under `t3-extension/`, with its
history, and is changed by items and reviewed like any other code here. Its
fixture test runs in CI on Linux against the Desk built from the same commit,
serving records the test owns, so a change to the Desk's contract that breaks
the extension fails the pull request that makes it.

The extension is released apart from `atelier`. A tag `t3-extension-v<version>`
on a commit of `master` publishes a GitHub release holding one bundle: the
extension's files and its installer, and a checksum. That release builds no
Rust, does not start the `atelier` release, and is never the repository's
"latest" release, which stays the newest `atelier` one. A device installs the
extension from a bundle and learns of a newer one from the releases, by the
tag's prefix.

The extension sits where the Desk does (ADR 0032): in this repository,
outside the `atelier` release. `atelier` itself still has no Node step in its
build or release; Node runs the extension's own tests and its release
workflow, and T3 Code's own runtime runs the extension on a device.

## Success criteria

- same-tree: the files under `t3-extension/` at the import's merge equal the
  tree of `spadaval/t3-code-atelier` at `a242410`, apart from line endings
  and trailing whitespace made consistent, and `git log --follow` on
  `t3-extension/custom.js` reaches the extension's first commit.
- contract-in-ci: on a pull request that renames one field the extension
  reads from `GET /api/sidebar`, the CI job for the extension fails, and on
  the base it passes. The job's run, from its start to its end, is under six
  minutes on a pull request that changes only `t3-extension/`.
- quiet-for-others: a pull request that changes only `t3-extension/` runs
  neither the `rust` job nor the `desk` job, and one that changes only
  `crates/atelier-core` does not run the extension's job.
  `scripts/ci-jobs.test.sh` covers both.
- own-release: the release workflow's no-publish run, started by hand on
  `master`, builds a bundle whose installer installs the extension into an
  empty home directory and whose files equal `t3-extension/` at that commit.
  `dist plan` for a tag `t3-extension-v0.1.1` does not plan an `atelier`
  release, and the workflow refuses a tag whose commit is not on `master`.
- installs-from-bundle: from the bundle alone, with no clone, the installer
  sets up a home directory in which `t3-custom check` passes, and the update
  command, given a fixture list of releases, picks the newest
  `t3-extension-v*` one, ignores `atelier`'s releases, verifies the bundle's
  checksum and refuses one that does not match. Tests cover each.

## Scope

In: `t3-extension/` (new), `.github/workflows/`, `scripts/ci-jobs.sh` and its
test, `dist-workspace.toml`, `AGENTS.md`, `README.md`, `docs/design.md` for
the decision, and `docs/product.md` or the Desk's documents where they name
the extension's repository.

Out, and done by the planner's thread after the items merge:

- Tagging the first extension release.
- Reinstalling the extension on Sumeet's Windows machine from the bundle, and
  running the fixture and contract tests there. No implementer on this host
  can run Windows or a real T3 Code.
- Archiving `spadaval/t3-code-atelier` with a pointer here, last.

Out altogether:

- Any change to what the extension shows or does, and any change to the
  Desk. atelier-zc0o, the fast-keys bug in mission control, is not part of
  this mission.
- Running the contract test, which needs a real T3 Code, on a schedule.
- macOS, and making T3 Code keep the extension across its own updates.

## Authority

An implementer may decide, and records in its Report: the layout inside
`t3-extension/`, the names of the workflow files and jobs, the bundle's
archive format and file names, how the extension's CI job gets its browser,
and how the `atelier` release is made to ignore the extension's tags.

An implementer asks the thread that started it before: changing the tag
scheme `t3-extension-v<version>`; changing how `atelier` itself is tagged,
built or released; letting a workflow publish from anything but a tag on
`master`; adding a dependency the extension must install on a device; or
making the device apply an update without the user having turned that on.

## Result
