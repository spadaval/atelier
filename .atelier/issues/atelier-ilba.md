---
created_at: "2026-11-07T05:01:00.583737629+00:00"
id: "atelier-ilba"
issue_type: "feature"
labels: []
priority: "P1"
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
title: "Refuse an invalid record when it is written, and let authors fix their own"
updated_at: "2026-11-07T05:01:00.583737629+00:00"
---

## Goal

Today a record that breaks a graph rule is written without complaint, then every later command refuses, and append-only rules stop its author fixing it. On 2026-11-05 this stalled astromech for 5 hours and ended in a force-push of main; cosplay-studio lost 8 dispositions and demiurge needed a throwaway issue. Check every write against the rules `atelier check` applies, and let an author correct or delete its own records until they reach the base.

## Done when

- [ ] Every record-writing command checks the records it writes against the graph rules `atelier check` applies, and refuses before writing. The test in `crates/atelier-cli/tests/cli_integration/discovered_from.rs` that asserts a disposition is accepted and `check` then fails asserts a refusal at write instead.
- [ ] A record that points at an issue existing only on another branch is refused at write, and the message names that branch.
- [ ] An author can edit or delete a review or evidence record it wrote while the record is not on the base; the append-only refusal applies only to records already on the base.
- [ ] Removed, and listed in the close report: append-only enforcement for records not yet on the base.

## Unknowns

- How to tell cheaply whether a record is already on the base.
- Whether the recorded identity is enough to allow an author to delete its record (identity is self-declared).
