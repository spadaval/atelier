---
schema: atelier.review
schema_version: 2
id: atelier-h6zc
title: finding on atelier-h3hl @1f516cb2
target: atelier-h3hl
identity: gpt-6.1-sol-h3hl-r1
commit: 1f516cb227031dded6728410b3b7ce5dc73c9d2f
created_at: 2026-11-11T07:09:57.057840294Z
event:
  event: finding
  recommendation: fix_now
---

medium: R1 next/crates/atelier-desk/src/acts.rs:122 Pending answers identify a decision only by item and question text (same at lines 151-152). The records allow repeated Decisions headings. In a scratch clone, commit an answered question and a later unanswered question with the same heading, then POST decide for the later question. The act returns done and writes the new answer without moving the branch, but GET item matches the earlier committed answer and removes the new pending answer. Both item and mission pending_answers are empty and the inbox lists the just-answered question again. This violates the pending-answer Done when. Retain the decision occurrence/index returned by question() as part of the internal pending identity, use it consistently for add, committed reconciliation and inbox filtering, and test repeated headings with both old committed and new uncommitted answers. Reproduced against a freshly built server with no T3 configuration; all scratch resources removed.
