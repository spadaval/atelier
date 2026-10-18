---
schema: atelier.review
schema_version: 1
id: atelier-z93s
title: 'Medium: crates/atelier-cli/src/commands/mission_review.rs:1451-1462 The HTML timeline shows a mission review event''s first_sentence(summary) only, and its record chip card also holds only the first sentence (RecordRef.summary is first_sentence), so everything after the first sentence of a plan approval or the human''s post-mission review is nowhere on the page; the Markdown timeline prints the full summary, and the previous HTML Mission reviews section printed the full event_line. Regression and Markdown/HTML disagreement on the very records that hold human direction. Fix: render the full summary (rich_inline, or a details element like receipts) and test a multi-sentence mission review in both forms.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:08.101110482Z
event:
  event: finding
  recommendation: fix_now
---

Medium: crates/atelier-cli/src/commands/mission_review.rs:1451-1462 The HTML timeline shows a mission review event's first_sentence(summary) only, and its record chip card also holds only the first sentence (RecordRef.summary is first_sentence), so everything after the first sentence of a plan approval or the human's post-mission review is nowhere on the page; the Markdown timeline prints the full summary, and the previous HTML Mission reviews section printed the full event_line. Regression and Markdown/HTML disagreement on the very records that hold human direction. Fix: render the full summary (rich_inline, or a details element like receipts) and test a multi-sentence mission review in both forms.
