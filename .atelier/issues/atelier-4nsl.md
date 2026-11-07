---
created_at: "2026-11-07T05:01:01.013293142+00:00"
id: "atelier-4nsl"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-4nsl
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8bh4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Review and merge a change that has no issue"
updated_at: "2026-11-07T10:31:01.480244835+00:00"
---

## Goal

A release, a docs edit or a mockup change should get an independent review without the issue around it: no create, start, Report or Done when. Sumeet called the release path "an insane amount of process", and an astromech agent moved mockup work out of the repository to avoid it. Let a branch that no issue owns be reviewed and merged on an approval of its head, under the same merge checks.

## Done when

- [x] A branch is "owned" when an issue that is not yet done on the base records it as its work branch (an owner already done on the base no longer owns the name). The base branch is never a light-path branch. On a branch other than the base that no issue owns, `atelier review request` with no id prints a brief for the branch's commits since it left the base (range, commit messages, diff stat, merge checks, working rules, and the commands to record findings and approval), with no Goal or Done when sections.
- [x] `atelier review finding`, `approve` and `disposition` can target such a branch (for example `--branch <name>` in place of an issue id), writing ordinary review records whose target names the branch. A branch-target record counts only when it is pinned to a commit after the branch left the base, so a later branch with a reused name (`release`) does not inherit an earlier one's findings or approval. Approvals stay append-only and go stale only on code changes, as for issues.
- [x] On such a branch, `atelier merge` with no id merges it after a current independent approval of its head, with its findings judged as for an issue (`findings.disposed`), under the same merge checks, and writes no issue state; without an approval it is refused. `atelier merge --sync` with no id works there too. `land` on the base afterwards counts its commits as reviewed. Tests for each.
- [x] Owned branches keep the issue path: on an owned branch, `merge` and `review request` with no id act on the owner, and branch-target records count for nothing. A branch approval covers no done claim: a light-path branch whose commits mark an issue done is refused by `claims.reviewed` in merge and by `land_base_done_claim` on the base. If `atelier start` takes a branch that has branch-target records, those records stop counting and the branch follows the issue path. Tests for each.
- [x] `atelier check`, `atelier doctor` and `land` on the base accept light-path records and judge them as merge does (test). Branch-target records are written at a newer review schema version, so an older binary reports "needs a newer Atelier" rather than an invalid record; issue-target records are still written at the current version (2), so astromech's pinned binary keeps reading every new ordinary review (test: a new approval of an issue is written at version 2).
- [x] A scenario test cuts a release this way: a branch with a version bump, notes, and the tag filed on that branch as a loose end with no blocker, is reviewed, approved and merged with no issue; the loose end reaches the base with the merge and is ready there.
- [x] `status` shows the current branch when it is unowned and not the base, and every unowned local branch that has branch-target records, each with its review state.
- [x] ADR 0028 (decision 6, decision 3's "left to atelier-4nsl" paragraph and its "blocked_by the issue or release" sentence, the ADR 0019 decision 3 entry), PRODUCT_INTENT.md's "explicit `start` and `merge`" principle, docs/product and CONTEXT.md describe the light path as built, and say when to use it rather than `review trivial --base`.
- [x] Narrowed, and listed in the implementer report: the rule that `merge` and `review request` need an owner issue now applies only to owned branches.

## Scope

`review request`, review record targets and schema, `merge` (including `--sync`), `land`'s base coverage, `doctor`, `status`, `start`'s interaction with branch-target records, docs, ADR 0028, PRODUCT_INTENT.md. Must not change: no non-trivial change reaches the base without an independent approval of its code; `review trivial --base` keeps covering one trivial commit made directly on the base. Other repositories keep working with the shared binary, and their issue-target review records are unchanged. astromech does not use the light path until atelier-ml3r moves its CI pin past this binary.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 6), docs/strategies/simpler-lifecycle.md (Shape: A light path), crates/atelier-records/src/review.rs (`deny_unknown_fields`, target validation), crates/atelier-app/src/record_index.rs (missing-target check), crates/atelier-cli/src/commands/git.rs (`BaseReviews`). atelier-md0v (start takes the current branch; branch names carry no type) lands first.

## Design

- The target is the branch name, stored in the existing review record format at a newer schema version (the implementer picks the field encoding). The approval covers the branch's code exactly as an issue approval does, but never a done claim.
- The tag's loose end is filed on the release branch with no blocker, so it reaches the base with the merge and cannot be forgotten afterwards (the v0.3.1 tag was lost because it lived in prose). It shows as ready on that branch too, which is harmless.
- Accepted risk, for Sumeet's after-merge read: a plain `git merge` of an approved light-path branch passes `land` without running `merge_checks`, because `land` cannot tell how a merge commit was made. The approval still covers the code; `atelier merge` is the documented path, and CI runs on the base.
- No trivial exemption for branches: an approval is the light path's only cover.

## Report

A branch no open issue owns now takes the light path: `atelier review request` with no ID briefs its commits since it left the base (range, subjects, diff stat, earlier findings and reviews of the branch, merge checks, working rules, `--branch` commands; no Goal or Done when), `review finding`, `approve` and `record` take `--branch <name>`, `review disposition` decides such a finding by its ID, and `atelier merge` (and `merge --sync`) with no ID merges it on a current independent approval of its head, judging its findings as `findings.disposed` does and running the base policy's merge checks, with a merge commit whose tree is Git's own merge. Records about a branch name `branch: <name>` at review schema version 3 and live in their own index map (`RecordIndex::branch_reviews`), so nothing that judges an issue reads them; issue records are still written at version 2. `land` on the base counts an accepted branch approval's commits as reviewed and still judges their done claims; `check` and `doctor` accept the records; `status` shows the checked-out branch's light-path state and other unowned branches with records. Proof: `cargo nextest run -p atelier-cli --test light_path` (8 scenario tests, one per Done when line or pair, including the release scenario), the `review::tests` unit tests in atelier-records (branch record round trip at version 3, issue record at version 2, an older reader's "needs a newer Atelier"), the full workspace suite with and without `test-hooks`, `cargo fmt -- --check`, and this binary's `atelier check` passing read-only in astromech, cosplay-studio, demiurge and life-on-rails. Three tests were checked against deliberately broken rules (the base coverage skip, the current-run filter, the owned-when-merged check) and failed as they should.

### Deviations

- Ownership has one definition everywhere: `session::branch_owners` now leaves out an owner done on the base, so `land`, `status` and `start`'s `start_branch_owned` follow it too, not only the light path. A branch whose recorded owner is done on the base can now be taken by `start`.
- `land` on an unowned branch, which the Scope lists only for base coverage, now treats the branch's own review records and the loose ends it files as carried with it rather than as untraceable tracker changes, says it merges on the light path, and reports `review.current` and `findings.disposed` once the branch holds records about itself. Without that, `land` and the Stop hook told a light-path branch to `atelier start` a loose end. A code-only unowned branch still lands with its old note, plus one sentence naming `atelier review request`, so the Stop hook stays quiet there.
- `review record --branch` is accepted too, since `record`, `approve` and `finding` share their arguments; `review disposition` takes no `--branch`.

### Decisions

- Encoding: a `branch` front-matter field in place of `target`, valid only from schema version 3; a branch record never parses as an issue record (`parse_review_record` refuses it), and a finding's disposition edit keeps its kind, so an on-base finding cannot be re-pointed from an issue to a branch of the same name.
- The current run: a record counts when HEAD holds its pin and the fork point does not. Records pinned to commits a rebase left behind count for nothing on the light path, rather than refusing as the issue path does, so a squash-merged earlier `release` cannot block every later one.
- On the base, a branch approval covers commits only when its pin reached the base through a merge (a pin on the base's first-parent line never counts), the merged branch head records no issue that names the branch and was open at the merge's first parent, the approval is as first committed, and every finding of its run up to the pin is settled.
- A merge check that reads an issue (`blockers.*`, `work.terminal`, `issue.sections_parseable`, `decision.recorded`) passes on the light path, since there is none; `shell.check`, `lint.none_blocking` and `git.worktree_clean` run as for an issue.
- A record about a branch is written only from a checkout of that branch, unowned and not the base, pinned after it left the base (`review_branch_not_checked_out`, `review_branch_owned`, `review_branch_base`, `review_branch_pin`).
- `start` taking a branch that holds records about itself prints a note that they stop counting; nothing else is needed, since ownership alone switches the branch to the issue path.

### Surprises

- `git_freshness::blob` takes only a full commit ID, so reading the base's copy through `refs/heads/<base>` silently found nothing and every done owner still owned its name; the scenario test caught it.
- Clap cannot take an optional positional before a required one, so `review record`, `approve` and `finding` take ID and SUMMARY as one positional list, as `review trivial` already did; `atelier review approve <id> --identity X "..."` is unchanged.

### Open items

- Narrowed: the rule that `merge` and `review request` need an owner issue now applies only to owned branches (ADR 0028 decision 6).
- One record at schema version 3 makes an older binary refuse to load the repository's records at all ("needs a newer atelier"), not just that record. astromech must not use the light path until atelier-ml3r moves its CI pin; in this repository, reinstall the binary (`cargo install --path crates/atelier-cli --locked`) right after this merges and before any light-path record lands.
- Accepted risk, for Sumeet's after-merge read: a plain `git merge` of an approved light-path branch passes `land` without `merge_checks` (ADR 0028 decision 6).
- Independent review is outstanding; the orchestrator arranges it.
