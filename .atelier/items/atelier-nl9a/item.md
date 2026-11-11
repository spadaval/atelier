---
schema: atelier.item
schema_version: 3
id: atelier-nl9a
title: Check base commits that reached the upstream without review, in CI
status: dropped
mission: null
labels:
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-30T06:52:06.059708538Z
---

## Goal

`land` on the base checks only the commits not yet on the upstream (`rev-list <tip> --not <upstream>` in crates/atelier-cli/src/commands/git.rs, where `BaseReviews::new(..., &upstream)` takes the same boundary), so once an unreviewed commit is pushed, past a gate whose refusal was masked by `| tail` or a `;` chain, nothing checks it again (incident 1620b534 on 2026-10-17 was found only because an agent noticed). Add `atelier land --since <commit>` that applies land's base rules to `<commit>..HEAD` instead of the unpushed range, and run it in this repository's CI on each push to master with the push's previous head, so a slipped push turns CI red within minutes. The red run is a notice, not a block: the commit is already on the base, and the fix is a reviewed follow-up, through an issue or the light path.

## Done when

- [ ] Integration tests with a bare origin: an unreviewed code commit pushed to the base passes plain `land` (unchanged behaviour), but `land --since <previous head>` refuses with `land_base_unreviewed` naming it.
- [ ] A range holding only record commits, merge commits `atelier merge` made on a current approval (of an issue, or of a light-path branch, atelier-4nsl) and trivially exempted commits passes; "reviewed" means approved code as `merge` judges it (atelier-3jdm).
- [ ] The CI job runs `atelier land --since <before>` on pushes to master and accepts a `since` input under `workflow_dispatch`; a dispatched run on a scratch branch whose range holds an unreviewed commit is shown failing, and one on master passing, with the run links in the Report.

## Scope

`land` in commands/session.rs and git.rs, the CI workflow, docs/product/sessions-and-hooks.md. Prevention is atelier-fad5's.

## Context

Kept in decision atelier-mzpn from the superseded atelier-ll3p: a CI notice that costs agents nothing and watches the one invariant atelier-mmpv keeps, that no unreviewed code reaches the base. Its receipt on that mission is dropped.

## Unknowns

- Base rules run only when the branch is the base, and an unowned branch without the base ref stops at `land_work_untraceable`, so `--since` must bypass that checkout rule for the scratch-branch proof.
- CI checks out a detached HEAD; `land --since` may need to accept one on the base commit.
- A first push or force push has no usable previous head (`github.event.before` is all zeros or unreachable); decide whether CI then checks from the last merge commit or skips with a visible note.

## Report

Dropped: `land --since` names a removed command. The pre-push hook now refuses the push itself, so a pipe cannot hide the refusal as in incident 1620b534. Refile with evidence if an unreviewed commit reaches master.
