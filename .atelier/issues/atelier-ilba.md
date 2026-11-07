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
status: "todo"
title: "Refuse an invalid record when it is written, and let authors fix their own"
updated_at: "2026-11-07T05:55:08.852925119+00:00"
---

## Goal

Today a record that breaks a graph rule is written without complaint, then every later command refuses, and append-only rules stop its author fixing it. On 2026-11-05 this stalled astromech for 5 hours and ended in a force-push of main; cosplay-studio lost 8 dispositions and demiurge needed a throwaway issue. Validate every write against the rules `atelier check` applies, and let records be corrected until they reach the base.

## Done when

- [ ] Every record-writing command validates the records it writes, together with the records they link to, against the rules `atelier check` applies, and refuses before writing anything. Tests cover rules that survive this mission, such as a finding with competing disposition events (crates/atelier-records/src/review.rs) and a link to a record that exists only on another branch. (The deferral source rule, `review_follow_up_source_mismatch`, is removed by atelier-md0v and needs no write-time version.)
- [ ] A record that points at an issue existing only on another branch is refused at write, and the message names that branch.
- [ ] A review or evidence record that is not yet on the base can be edited or deleted on its branch; the append-only refusal applies only to records whose path exists at the tip of the policy's base branch. Test: write a finding, delete it in a later commit, and `atelier check`, `merge --check` and `land` all accept the branch.
- [ ] Removed, and listed in the implementer report: append-only enforcement for records not yet on the base.

## Scope

Record writes in `issue`, `review` and `evidence` commands; the append-only check; tests and docs/product. Must not change: append-only for records already on the base; the record formats (no migration needed).

## Context

docs/adr/0028-simpler-lifecycle.md (decision 4), crates/atelier-cli/tests/cli_integration/discovered_from.rs (today's accept-then-refuse test), crates/atelier-app/src/review_gate.rs (`review_change_allowed`, used by the append-only check).

## Design

- "On the base" means the record's path exists in the tip of the policy's base branch; one `git cat-file` per record is cheap enough.
- No identity rule: before a record reaches the base, any agent on the branch may correct it. Identity is self-declared, so an author check would add a rule without stopping anyone.
