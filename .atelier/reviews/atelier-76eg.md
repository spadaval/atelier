---
schema: atelier.review
schema_version: 1
id: atelier-76eg
title: approval on atelier-ov97 @a8b85019
target: atelier-ov97
identity: claude-review-ov97
commit: a8b85019bc2b8382b16110557f7b5a1cb8b9c12b
created_at: 2026-11-05T02:39:29.698686376Z
event:
  event: approval
---

Read the whole branch diff against S4, mission-reviews.md and the obcr receipt's first failure. Both forms now show each done child's Outcome, Changed, Surprised by and Left open under its child line (Markdown at 4-space indent, HTML under the chip). Behaviour changes beyond Done when belong here: a parent now changes course on a child's reported Deviations, the HTML entry stops being quiet for the same reason, and quiet deviations= counts child Deviations (r6bt 24 -> 30). Derived child facts were already counted as the parent's, so this keeps Plan vs Actual and the counts in line with the Changed lines the timeline now shows. mission-reviews.md describes it. Ran: diff --check, fmt, metadata --locked clean; atelier-app 107/107; atelier-cli debrief|brief 40/40; branch binary debrief atelier-r6bt shows w5hx's Surprises and Left open under its child line, and the --html page holds the same text; vt8q runs in 1.2s, both forms. PR #25 CI ubuntu and macos test jobs pass at a8b85019. One deferred finding, atelier-dswi (sibling follow-ups hide a child's Open items). Proof gaps (not blocking): the fixture does not check the quiet count or the HTML notable change.
