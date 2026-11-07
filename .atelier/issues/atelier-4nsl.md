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
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Review and merge a change that has no issue"
updated_at: "2026-11-07T05:01:01.013293142+00:00"
---

## Goal

A release, a docs edit or a mockup change should get reviewed without the issue, start and close ceremony around it. Sumeet called the release path "an insane amount of process", and an astromech agent moved mockup work out of the repository to avoid it.

## Done when

- [ ] A branch with no owning issue merges with `atelier merge` after an independent approval of its head, under the same merge checks; without one it is refused.
- [ ] A scenario test cuts a release this way: version bump and notes reviewed and merged, the tag filed as a loose end.
- [ ] `status` shows such a branch and its review state.

## Unknowns

- How a review targets a branch rather than an issue.
