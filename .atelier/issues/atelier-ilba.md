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
updated_at: "2026-11-07T06:05:08.500208172+00:00"
---

## Goal

Today a record that breaks a graph rule is written without complaint, then every later command refuses, and append-only rules stop its author fixing it. On 2026-11-05 this stalled astromech for 5 hours and ended in a force-push of main; cosplay-studio lost 8 dispositions and demiurge needed a throwaway issue. Validate every write against the rules `atelier check` applies, and let review records be corrected until they reach the base.

## Done when

- [ ] Every command that writes records (including `start`, `merge`, `migrate` and `prune`) validates the records it writes, together with the records they link to, against the rules `atelier check` applies, and refuses before writing anything. Tests cover rules that survive this mission, such as a finding with competing disposition events (crates/atelier-records/src/review.rs) and a link to a record that exists only on another branch. (The deferral source rule, `review_follow_up_source_mismatch`, is removed by atelier-md0v and needs no write-time version.)
- [ ] A record that points at an issue existing only on another branch is refused at write, and the message names that branch.
- [ ] A review record can be edited or deleted until it reaches the base. Each edit is judged against the base as it stood where the edit joined it: for a commit that reached the base through a merge, whether the path existed at that merge's first parent; for an unmerged commit, whether it exists at the tip of the policy's base branch. Tests: write a finding on a branch, edit it in a later commit, delete another, merge; `atelier check`, `merge --check`, `land` and `doctor` all pass, on the branch and on the base afterwards. Editing a review record already on the base is still refused.
- [ ] ADR 0028's decision 4 paragraph and PRODUCT_INTENT.md's "Guard valid records ..." and "Keep review and evidence append-only ..." principles describe the rule as built, including the risk noted below.
- [ ] Removed, and listed in the implementer report: append-only enforcement for review records not yet on the base.

## Scope

Every record-writing command; the append-only check in `land`, `doctor` and merge gates; tests, docs/product, ADR 0028, PRODUCT_INTENT.md. Must not change: append-only for review records already on the base; the record formats (no migration needed). Evidence has no append-only check today and gets none.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 4), crates/atelier-app/src/review_gate.rs (`review_change_allowed` and the append-only check), crates/atelier-app/src/git_freshness.rs (`edges`), crates/atelier-cli/src/commands/session.rs (`land`).

## Design

- "On the base" is judged per edit, as in Done when, so a correction made on a branch and then merged is not refused afterwards; one `git cat-file` per record and edge is cheap enough.
- No identity rule: before a record reaches the base, any agent on the branch may correct it. Identity is self-declared, so an author check would add a rule without stopping anyone. Risk, accepted and noted in the ADR (planning review finding R13): an implementer could delete a reviewer's finding before merge, so no decision on it is recorded; it cannot let unreviewed code merge, since approvals still cover the code.
