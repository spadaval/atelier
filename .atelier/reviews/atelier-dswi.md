---
schema: atelier.review
schema_version: 1
id: atelier-dswi
title: finding on atelier-ov97 @a8b85019
target: atelier-ov97
identity: claude-review-ov97
commit: a8b85019bc2b8382b16110557f7b5a1cb8b9c12b
created_at: 2026-11-05T02:30:21.586859406Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/mission_review.rs:954 and :1938 pass entry.follow_ups.is_empty() to report_facts for every child, but WorkEntry.follow_ups aggregates the follow-ups review filed on the parent and all its children (atelier-app mission_review.rs work_entry members loop). So review filing a follow-up on child A hides child B's unrelated Open items in both forms, and the parent's Filed list does not cover them: Open items B reported appear nowhere, the same loss this bug fixes. No real mission hits it today (r6bt has no follow-ups; vt8q's entries with follow-ups have no children), and the fixture has no follow-ups. Suggested fix: carry each child's own follow-ups (or a has_follow_ups flag) on WorkChild and gate that child's Left open on it, with a fixture where only one child's review files a follow-up.
