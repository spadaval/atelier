---
created_at: "2026-11-07T05:01:01.013293142+00:00"
id: "atelier-4nsl"
issue_type: "feature"
labels: []
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
status: "todo"
title: "Review and merge a change that has no issue"
updated_at: "2026-11-07T10:01:26.261835460+00:00"
---

## Goal

A release, a docs edit or a mockup change should get an independent review without the issue around it: no create, start, Report or Done when. Sumeet called the release path "an insane amount of process", and an astromech agent moved mockup work out of the repository to avoid it. Let a branch that no issue owns be reviewed and merged on an approval of its head, under the same merge checks.

## Done when

- [ ] On a branch no issue owns, `atelier review request` with no id prints a brief for the branch's commits since it left the base (range, commit messages, diff stat, merge checks, working rules, and the commands to record findings and approval), with no Goal or Done when sections.
- [ ] `atelier review finding`, `approve` and `disposition` can target such a branch (for example `--branch <name>` in place of an issue id), writing ordinary review records whose target names the branch; approvals stay append-only and go stale only on code changes, as for issues.
- [ ] On such a branch, `atelier merge` with no id merges it after a current independent approval of its head with every finding decided, under the same merge checks, and writes no issue state; without an approval it is refused. `land` on the base afterwards counts its commits as reviewed. Tests for both.
- [ ] A scenario test cuts a release this way: a branch with a version bump and notes is reviewed, approved and merged with no issue, and the tag is then filed as a loose end with no blocker, since it is filed after the merge it follows.
- [ ] `status` shows such a branch with its review state.
- [ ] `review trivial --base` stays as it is: it covers one trivial commit made directly on the base, which is a different case. Docs say when to use which.
- [ ] Removed, and listed in the implementer report: the rule that `merge` and `review request` need an owner issue.
- [ ] ADR 0028 decision 6, docs/product and CONTEXT.md describe the light path as built.

## Scope

`review request`, review record targets, `merge`, `land`'s base coverage, `status`, docs, ADR 0028. Must not change: no non-trivial change reaches the base without an independent approval of its code. Other repositories keep working with the shared binary; their review records with issue targets are unchanged.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 6), docs/strategies/simpler-lifecycle.md (Shape: A light path). atelier-md0v (start takes the current branch; branch names carry no type) lands first, so `start` must not claim a branch the light path is using.

## Design

- The target is the branch name, stored in the existing review record format (the implementer picks the field encoding; old records stay valid). The approval covers the branch's code exactly as an approval of an issue does.
- What a loose end waits on after an issue-less change: nothing. It is filed once the change it follows has merged, so it needs no blocker.
