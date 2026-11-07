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
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Adopt and upgrade Atelier without hand repair"
updated_at: "2026-11-07T05:01:01.226766008+00:00"
---

## Goal

Life-on-rails hit about ten refusals between `atelier init` and its first merge, and cosplay-studio needed a hand-written migrator after four dormant months. Make a new repository merge with the starter policy as written, and give every schema change a migration.

## Done when

- [ ] In a fresh repository, with and without CI, the policy `atelier init` writes lets the lifecycle scenario merge with no policy edits.
- [ ] Every record schema version still present on this host migrates with `atelier migrate`, each covered by a test.
- [ ] Untracked files outside `.atelier/` do not block `merge` or `land`.
