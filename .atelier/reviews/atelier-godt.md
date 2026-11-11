---
schema: atelier.review
schema_version: 2
id: atelier-godt
title: finding on atelier-a4rs @753b046c
target: atelier-a4rs
identity: GPT-6.1 Sol independent planning reviewer
commit: 753b046cec958c26e0504daaf8cb680ba44f181c
created_at: 2026-11-11T04:33:33.833107326Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-a4rs.md:48 (753b046c): medium, FIX NOW. The next-decide pass rule requires decide --defer while also requiring that the decider did not edit the item. In the new tool the answer and Outcome: defer are edits of that very item.md: next/crates/atelier-cli/src/questions.rs:127-141 changes dir.item.body and calls write_item; tests/decisions.rs:171-176 checks those added lines. A literal state grader would fail the intended passing path, while treating any edit as allowed could miss an unauthorized change of Goal, Done when or Scope. State the allowed delta explicitly: only the answer/outcome of the seeded Decisions entry may change, with no start/takeover, definition or code change. Require the scripted pass to include that normal decide write and the failing fixture to exercise an unauthorized edit.
