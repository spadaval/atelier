---
created_at: "2026-11-07T05:01:01.226766008+00:00"
id: "atelier-l457"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4nsl"
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
title: "Adopt and upgrade Atelier without hand repair"
updated_at: "2026-11-07T10:09:45.953366781+00:00"
---

## Goal

Life-on-rails hit about ten refusals between `atelier init` and its first merge, and cosplay-studio needed a hand-written migrator after four dormant months. Make a new repository merge with the starter policy as written, make the first steps after `init` obvious, and give every schema change a migration.

## Done when

- [ ] In a fresh Git repository, with and without CI, the policy `atelier init` writes lets the lifecycle scenario (crates/atelier-cli/tests/lifecycle_cost.rs's steps) merge with no policy edits. `init` sets `base_branch` to the branch it finds (`main`, `master` or the current branch), not a fixed name.
- [ ] `atelier init` in a directory that is not a Git repository refuses with a message saying to run `git init` first; in a repository with an unborn HEAD (no commits yet), `init`, `status`, `check` and `land` work or say plainly what the first commit needs. Tests for each.
- [ ] `atelier init` ends by printing what it wrote and the next steps in the new lifecycle: commit the install, get it reviewed on a branch (the light path, atelier-4nsl), merge; then create the first issue.
- [ ] Every record schema version still present in any repository on this host migrates with `atelier migrate`, each covered by a test, and `atelier check` names the migrate command when it meets an old version it can still read.
- [ ] Untracked files outside `.atelier/` do not block `merge` or `land` (test with an untracked file at the repository root).
- [ ] Removed, and listed in the implementer report: whatever init-time or adoption-time refusal these tests show to be unnecessary.

## Scope

`init`, the starter policy, `migrate`, `check`'s version messages, `merge` and `land`'s checkout-cleanliness checks, tests, docs/product. Must not change: refusing uncommitted changes to tracked files, which could make merge or evidence lie about what was reviewed.

## Context

docs/strategies/simpler-lifecycle.md (Shape: Small repositories work out of the box), ADR 0028 decision 7. The draft-mission items atelier-0vp3 (base detection), atelier-6tmk (no Git, unborn HEAD) and atelier-t3ko (init next steps) cover the same ground; whether they are superseded by this item is part of Sumeet's decision on atelier-mzpn. atelier-md0v (no recorded base) and atelier-4nsl (light path) land first.
