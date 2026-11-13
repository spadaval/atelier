---
schema: atelier.item
schema_version: 3
id: atelier-t5am
title: Finish a merge through the forge's pull request, so Atelier works on a protected base
status: draft
mission: atelier-to83
labels:
- redesign
priority: medium
blocked_by:
- atelier-ob9u
- atelier-2vdq
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T06:14:32.695875418Z
---

## Goal

Make `atelier merge` work where the base is protected. With a forge
configured, `merge` builds the merge commit itself and pushes the base, which
a protected branch refuses. After this item it finishes through the forge's
merge of the pull request, and the pre-push hook's rule becomes the plain one:
the base receives only merges.

## Done when

- [ ] With a forge configured, `merge` does its checks, the plan-review gate and the record-only rule of atelier-2vdq and atelier-ob9u among them, and then has the forge merge the pull request, as a merge commit. It never pushes the base. With no forge it moves the base locally as today.
- [ ] `done` is recorded as `docs/design.md` settled, and after a merge by the forge `atelier show <id>` in a fresh clone says done.
- [ ] The merge checks in `config.toml` still run on the merged tree before the forge is asked, and the Report says what can change between that run and the forge's merge and what catches it.
- [ ] Where the forge will not merge yet because a required review or check is missing, `merge` stops with exit status 3 and says which, and run again later it merges.
- [ ] A test remote whose hook refuses every push to the base lets the full item loop pass: `start`, `submit`, `review`, `verdict`, `merge`. The forge's side is the existing contract-test double.
- [ ] The pre-push hook refuses any push that moves the base, and says to use `atelier merge`. `ATELIER_SKIP` passes it with a reason, as now.
- [ ] The merge lock and the checks that the branch and base have not moved are kept, with their tests, or the Report says which the forge now makes unnecessary and why.

## Scope

In: `crates/atelier-cli` (`merge.rs`, `forge.rs`, `check.rs`), their tests, and the Merge, Hooks and Forge sections of `docs/design.md` where the code differs from what atelier-ogpg wrote.

Out: turning on branch protection anywhere; any forge but GitHub; a merge queue; squash or rebase merges.

## Context

`docs/design.md`, Merge, steps 5 to 8: `merge` writes the merge commit with the item set `done`, moves the base and pushes it. The trailer `Atelier-Merge: <id>` is how the pre-push hook knows an Atelier merge today.
