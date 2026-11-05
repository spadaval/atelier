---
schema: atelier.review
schema_version: 1
id: atelier-nu4k
title: finding on atelier-2bjz @6ea3eb5f
target: atelier-2bjz
identity: claude-review-2bjz
commit: 6ea3eb5f5c3cd2e45f14349fa20da5d4d957a7d1
created_at: 2026-11-04T23:37:19.429119065Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-core/src/lib.rs:946-958 misspelled_label treats any continuation-line Word: within two Levenshtein edits of reason/rejected as a typo, so ordinary prose lines are refused: Person:, Region:, Lesson:, Season: (2 or fewer edits from reason) and Selected:, Detected: (2 from rejected). A hand-edited decision note with such a line now fails check (and so land) after an upgrade, in any repository. The juyb decision note accepts rare false refusals, but these words are common. Suggested: case-insensitive exact match, or Damerau-Levenshtein distance 1 (still catches Rejectd, Reasons, Raeson, reason, REJECTED in the unit test), and add Person:/Selected: to the allowed-prose unit cases.
