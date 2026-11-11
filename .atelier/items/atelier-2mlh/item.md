---
schema: atelier.item
schema_version: 3
id: atelier-2mlh
title: Run the first real items under the new tool, and fix what first use breaks
status: building
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by:
- atelier-wd1i
implementers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A77050191-cad3-4b48-9372-f665e6d5a28e
ready:
  by: sumeet
  at: 2026-11-11T15:43:38.001833021Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.176751966Z
---

## Goal

Take the first real item through the new tool in this repository, and with it remove the last piece of the old one from CI. CI's `check` step still has a branch that downloads the old 0.4.0 release for a repository that holds `.atelier/issues/`; no repository does any more. This item is also the first time the new binary pushes a branch, opens a pull request, waits for CI and pushes `master` on real GitHub, so what it shows about the loop is half its result.

## Done when

- [ ] `.github/workflows/ci.yml`'s `check` step builds and runs the tree's binary only: it downloads no release, holds no checksum, and does not look for `.atelier/issues/`. The step's commands, run by hand in a throwaway clone of this branch, pass, and the Report gives them with their output.
- [ ] Nothing else in `.github/` or `scripts/` outside `scripts/cutover/` still serves the fallback: `git grep -n -e 'v0\.4\.0' -e '\.atelier/issues' -- .github scripts ':!scripts/cutover'` prints nothing, or the Report explains each line it prints. `scripts/cutover/` is left alone: its rollback and its `adopt` command still need the fallback until the work host has adopted.
- [ ] The Report keeps the record of this first use as far as it can be known before `submit`: each `atelier` command run so far, in order, with its exit status, and every refused or errored call, with what it printed. What happens from `submit` on (the push, the pull request, CI on it, the reviews, `merge` and the push of `master`) cannot be ticked here, since `submit` checks these lines first. The implementer reports it to the mission's lead when the item is done, and the lead writes it into the mission's Result. Throughout, the branch is not pushed, the pull request not opened and `master` not pushed by hand around the tool.
- [ ] Each defect this first use showed in the binary, the skill or `AGENTS.md` before `submit` is in the Report with what was seen. None is fixed here, since each is outside this item's Scope: one that stops the item is put to the lead with `atelier escalate`, and the rest are left for the lead to file.
- [ ] The Report gives the non-test Rust line count of each crate under `crates/` (`scripts/count_non_test_rust.py`), beside the 48,988 lines the old workspace had before the redesign.

## Scope

`.github/workflows/ci.yml`, a script it calls where that script serves only the fallback, and this item's record. Not `scripts/cutover/`, not the docs (atelier-6w8d), and not what `status` prints (atelier-ivbq).

## Context

Mission atelier-qr5s, step H. The cutover ran on 2026-11-11: this repository's records are in the new layout, the installed `atelier` is 0.5.0 built from this tree, and `AGENTS.md` describes the loop. The specifications are `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

The same step asks for one item in a product repository. That is worldcrafter-s9lp in demiurge, planned there with the same care: the cutover moved that repository's open mission's strategy into the mission's directory, and tracked files outside `.atelier/` still name the old path. Its Done when is that a search for the old path finds nothing outside `.atelier/`, each changed reference resolves, and the change touches nothing else. demiurge configures no forge, so its merge is local and its base is pushed with Git, as its `AGENTS.md` says.

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`; set `TMPDIR` to a directory under `$HOME` outside any repository, since `/tmp` is a small quota'd tmpfs.
