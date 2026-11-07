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

- [ ] A branch is "owned" when an issue that is not yet done on the base records it as its work branch (an owner already done on the base no longer owns the name). The base branch is never a light-path branch. On a branch other than the base that no issue owns, `atelier review request` with no id prints a brief for the branch's commits since it left the base (range, commit messages, diff stat, merge checks, working rules, and the commands to record findings and approval), with no Goal or Done when sections.
- [ ] `atelier review finding`, `approve` and `disposition` can target such a branch (for example `--branch <name>` in place of an issue id), writing ordinary review records whose target names the branch. A branch-target record counts only when it is pinned to a commit after the branch left the base, so a later branch with a reused name (`release`) does not inherit an earlier one's findings or approval. Approvals stay append-only and go stale only on code changes, as for issues.
- [ ] On such a branch, `atelier merge` with no id merges it after a current independent approval of its head, with its findings judged as for an issue (`findings.disposed`), under the same merge checks, and writes no issue state; without an approval it is refused. `atelier merge --sync` with no id works there too. `land` on the base afterwards counts its commits as reviewed. Tests for each.
- [ ] Owned branches keep the issue path: on an owned branch, `merge` and `review request` with no id act on the owner, and branch-target records count for nothing. A branch approval covers no done claim: a light-path branch whose commits mark an issue done is refused by `claims.reviewed` in merge and by `land_base_done_claim` on the base. If `atelier start` takes a branch that has branch-target records, those records stop counting and the branch follows the issue path. Tests for each.
- [ ] `atelier check`, `atelier doctor` and `land` on the base accept light-path records and judge them as merge does (test). Branch-target records are written at a newer review schema version, so an older binary reports "needs a newer Atelier" rather than an invalid record; issue-target records are still written at the current version (2), so astromech's pinned binary keeps reading every new ordinary review (test: a new approval of an issue is written at version 2).
- [ ] A scenario test cuts a release this way: a branch with a version bump, notes, and the tag filed on that branch as a loose end with no blocker, is reviewed, approved and merged with no issue; the loose end reaches the base with the merge and is ready there.
- [ ] `status` shows the current branch when it is unowned and not the base, and every unowned local branch that has branch-target records, each with its review state.
- [ ] ADR 0028 (decision 6, decision 3's "left to atelier-4nsl" paragraph and its "blocked_by the issue or release" sentence, the ADR 0019 decision 3 entry), PRODUCT_INTENT.md's "explicit `start` and `merge`" principle, docs/product and CONTEXT.md describe the light path as built, and say when to use it rather than `review trivial --base`.
- [ ] Narrowed, and listed in the implementer report: the rule that `merge` and `review request` need an owner issue now applies only to owned branches.

## Scope

`review request`, review record targets and schema, `merge` (including `--sync`), `land`'s base coverage, `doctor`, `status`, `start`'s interaction with branch-target records, docs, ADR 0028, PRODUCT_INTENT.md. Must not change: no non-trivial change reaches the base without an independent approval of its code; `review trivial --base` keeps covering one trivial commit made directly on the base. Other repositories keep working with the shared binary, and their issue-target review records are unchanged. astromech does not use the light path until atelier-ml3r moves its CI pin past this binary.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 6), docs/strategies/simpler-lifecycle.md (Shape: A light path), crates/atelier-records/src/review.rs (`deny_unknown_fields`, target validation), crates/atelier-app/src/record_index.rs (missing-target check), crates/atelier-cli/src/commands/git.rs (`BaseReviews`). atelier-md0v (start takes the current branch; branch names carry no type) lands first.

## Design

- The target is the branch name, stored in the existing review record format at a newer schema version (the implementer picks the field encoding). The approval covers the branch's code exactly as an issue approval does, but never a done claim.
- The tag's loose end is filed on the release branch with no blocker, so it reaches the base with the merge and cannot be forgotten afterwards (the v0.3.1 tag was lost because it lived in prose). It shows as ready on that branch too, which is harmless.
- Accepted risk, for Sumeet's after-merge read: a plain `git merge` of an approved light-path branch passes `land` without running `merge_checks`, because `land` cannot tell how a merge commit was made. The approval still covers the code; `atelier merge` is the documented path, and CI runs on the base.
- No trivial exemption for branches: an approval is the light path's only cover.
