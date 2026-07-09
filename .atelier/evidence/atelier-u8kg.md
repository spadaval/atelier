---
created_at: "2026-07-06T18:09:39.086729673+00:00"
id: "atelier-u8kg"
evidence_type: "audit"
captured_at: "2026-07-06T18:09:39.086706808+00:00"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent PR 47 review result: fail/request changes. Finding P1: provider review show --comments fails live on PR 47 with Forgejo 404 GetReviewByID; command transcript is evidence atelier-9xvc. Finding P1: review submit --approve serializes APPROVE at crates/atelier-app/src/forgejo.rs:454, but provider acceptance requires APPROVED; existing mock asserts the wrong payload while supplying an independent APPROVED response. Finding P2: docs/adr/0011-native-review-modes-and-room-authority.md:28 retains the removed link verb instead of review open --existing. Provider review #32 requested changes; provider comment #230 records the approval finding. Focused 31-test review/provider/room suite passed; cargo fmt -- --check, git diff --check, and atelier check passed. Full atelier-app/atelier-cli suite produced the two known mission-baseline global-HOME/dirty-fixture failures, unchanged code paths, with 556 other tests passing."
updated_at: "2026-07-06T18:09:45.003476839+00:00"
---

Independent PR 47 review result: fail/request changes. Finding P1: provider review show --comments fails live on PR 47 with Forgejo 404 GetReviewByID; command transcript is evidence atelier-9xvc. Finding P1: review submit --approve serializes APPROVE at crates/atelier-app/src/forgejo.rs:454, but provider acceptance requires APPROVED; existing mock asserts the wrong payload while supplying an independent APPROVED response. Finding P2: docs/adr/0011-native-review-modes-and-room-authority.md:28 retains the removed link verb instead of review open --existing. Provider review #32 requested changes; provider comment #230 records the approval finding. Focused 31-test review/provider/room suite passed; cargo fmt -- --check, git diff --check, and atelier check passed. Full atelier-app/atelier-cli suite produced the two known mission-baseline global-HOME/dirty-fixture failures, unchanged code paths, with 556 other tests passing.
