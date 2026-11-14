---
schema: atelier.review
schema_version: 3
round: 2
commit: 9492ec2e05038ec5fe5e93f426e31d5dace9c037
reviewers:
- thread:delegated-task:command%3Amcp%3A524904cd-3475-4a13-9587-b5a6781d23a6%3Adelegate-task%3Abff57233-8825-496b-8ac0-c381c48cd3f2
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T06:18:57.089802049Z
---

## Summary

Complexity: low. The release edits the version, lockfile, changelog and one
cutover instruction without changing runtime code.
Value: high. Release installers need the workflow already used on this host.
Risk of merging: medium. Incorrect upgrade instructions would reach users,
although the release metadata and notes are easy to correct.
The value warrants this scope; a smaller change would not supply the release.

Reviewed 9492ec2e05038ec5fe5e93f426e31d5dace9c037 from a fresh detached worktree
at origin/atelier-6hb2. Compared the release diff with master, the round 2
change with the previous reviewed commit, and the changelog claims with
`git diff v0.5.0 origin/atelier-6hb2`. Checked command definitions, branch
naming and discovery, record writes, mission compatibility and plan gates,
Git-backed history, disputed findings, forge merging, hooks and the status
changes. The listed capabilities and command removals are implemented. The
upgrade section correctly distinguishes agreed missions from old drafts and
explains how to continue an existing draft without creating another mission.
No additional upgrade requirement was found.

The four versioned tracker packages and locked Cargo metadata agree at
0.6.0; the unreleased atelier-desk package intentionally remains 0.0.0.
The Report's reference to five versioned packages is a counting error,
with no effect on the release. The candidate binary prints `atelier 0.6.0`.
No stale current-version declaration was found. The 0.5-installed hook
scripts still invoke `atelier check --hook` unchanged.

Validation: 92 integration tests passed across branches, planning, hooks,
disputes and log; one performance test is ignored by default. The additional
protected-base item-loop test passed with the forge stub, bringing the total
to 93. Locked Cargo metadata, formatting and release-diff whitespace checks
passed. All test processes used
TMPDIR=/home/supadava/.cache/atelier-review-6hb2-r2.

A separate scratch reproduction used 0.5-style mission records without
`planners`. For the draft, the changelog's worktree command followed by
`submit`, two plan approvals from separate detached checkouts, and `merge`
preserved its ID and put it on the base as `open`. Old `ready` and `open`
missions passed `check` and could be led on a backlog branch. I did not
perform a live forge merge, tag a release or build release archives.
`atelier review` refused the detached checkout and directed review writes
to the branch's existing checkout; only this review file was edited there.

## Finding 1: Explain how to continue an existing 0.5 draft mission
Recommendation: fix now
Was: 1-amhk.1
Response: fixed in 342759dbe. The changelog has an Upgrading from 0.5 section: a `ready` or `open` mission carries on, a `draft` one continues on a branch named for its ID through `submit` and two plan approvals, and `ready` takes items only.
Now: fixed

CHANGELOG.md:14-22 now states that old ready/open missions are agreed, that
an old draft needs its existing ID's branch, submission and two plan
approvals before merging, that `start --mission` creates a different mission,
and that `ready` accepts items only. The old-draft regression test and the
separate reproduction above confirm the documented continuation path.
