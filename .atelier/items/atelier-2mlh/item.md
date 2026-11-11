---
schema: atelier.item
schema_version: 3
id: atelier-2mlh
title: Run the first real items under the new tool, and fix what first use breaks
status: draft
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by:
- atelier-wd1i
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.176751966Z
---

## Goal

Take the first real item through the new tool in this repository, and with it remove the last piece of the old one from CI. CI's `check` step still has a branch that downloads the old 0.4.0 release for a repository that holds `.atelier/issues/`; no repository does any more. This item is also the first time the new binary pushes a branch, opens a pull request, waits for CI and pushes `master` on real GitHub, so what it shows about the loop is half its result.

## Done when

- [ ] `.github/workflows/ci.yml`'s `check` step builds and runs the tree's binary only: it downloads no release, holds no checksum, and does not look for `.atelier/issues/`. CI passes on this item's pull request with that step running `atelier check`.
- [ ] Nothing else in `.github/` or `scripts/` outside `scripts/cutover/` still serves the fallback: `git grep -n -e 'v0\.4\.0' -e '\.atelier/issues' -- .github scripts ':!scripts/cutover'` prints nothing, or the Report explains each line it prints. `scripts/cutover/` is left alone: its rollback and its `adopt` command still need the fallback until the work host has adopted.
- [ ] The item went from ready to done by the tool's commands on real GitHub, and the Report gives the record of it: each `atelier` command run, in order, with its exit status; the pull request; the CI run `merge` waited for; and every refused or errored call, with what it printed. The branch was not pushed, the pull request not opened and `master` not pushed by hand around the tool.
- [ ] Each defect this first use showed in the binary, the skill or `AGENTS.md` is in the Report with what was seen. One that stops the item, is small and is inside the specifications is fixed here with a test; anything else is left for the mission's lead and not worked around.
- [ ] The Report gives the non-test Rust line count of each crate under `crates/` (`scripts/count_non_test_rust.py`), beside the 48,988 lines the old workspace had before the redesign.

## Scope

`.github/workflows/ci.yml`, a script it calls where that script serves only the fallback, and this item's record. Not `scripts/cutover/`, not the docs (atelier-6w8d), and not what `status` prints (atelier-ivbq).

## Context

Mission atelier-qr5s, step H. The cutover ran on 2026-11-11: this repository's records are in the new layout, the installed `atelier` is 0.5.0 built from this tree, and `AGENTS.md` describes the loop. The specifications are `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

The same step asks for one item in a product repository. That is worldcrafter-s9lp in demiurge, planned there with the same care: the cutover moved that repository's open mission's strategy into the mission's directory, and tracked files outside `.atelier/` still name the old path. Its Done when is that a search for the old path finds nothing outside `.atelier/`, each changed reference resolves, and the change touches nothing else. demiurge configures no forge, so its merge is local and its base is pushed with Git, as its `AGENTS.md` says.

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`; set `TMPDIR` to a directory under `$HOME` outside any repository, since `/tmp` is a small quota'd tmpfs.
