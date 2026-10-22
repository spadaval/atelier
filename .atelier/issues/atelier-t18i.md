---
created_at: "2026-10-17T06:28:32.644079557+00:00"
id: "atelier-t18i"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
  - kind: "review"
    id: "atelier-472z"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-9839"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Count links and name blocker cycles the same way for every link storage"
updated_at: "2026-10-17T06:28:32.644079557+00:00"
---

## Goal

After atelier-ihss, links can be stored on the declaring record, on the blocker's side (`blocks`), or on both (legacy). Two outputs still depend on that storage (review of atelier-ihss):
- The Relationships line `issue show` prints for missions and parents (`crates/atelier-app/src/issue_read.rs`, ~171-193): the "other" count counts a context link 3 times when stored on both records and 2 when stored on one (atelier-o3lm shows 32 on master, 31 after ihss), and "open blockers" now also counts a mission's own blockers stored on the blocker's record (finding atelier-9839).
- The cycle error (`crates/atelier-app/src/record_index.rs`, ~360-363) says `relationships.blocks contains a cycle`, but new links are stored as `blocked_by` in `relates`, so a reader looking in `blocks` finds nothing (finding atelier-472z).

## Done when

- Each link counts once in the Relationships line, whatever its storage, shown by a test with declared, blocker-side and two-sided links.
- The cycle error names the issues in the cycle and the link role, not a storage field.
