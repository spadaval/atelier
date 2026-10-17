---
schema: atelier.review
schema_version: 1
id: atelier-ncba
title: 'crates/atelier-cli/src/commands/mission_review.rs:644-651: a review or evidence record mentioned in record text is rendered with its ID as the visible label (the popover gives kind and summary). On atelier-r6bt the Work entries for sfsq, ho7o and rx22 read ''review atelier-tqgi showed...'', ''Review atelier-dqty...'', ''Review atelier-gq88 and atelier-yx83...'', which the human reads as bare IDs, against Done when''s ''no bare IDs in the HTML text'' and the mockup, whose Changed lines name no IDs. Issue references are correct. Fix: label a record mention by its kind and a short summary (for example ''a finding'' or the first words of its summary) and keep the ID only in the popover; add an assertion to the HTML fixture.'
target: atelier-nexd
identity: Independent reviewer 2 (Claude subagent)
commit: 1d145e0d2c4043e778409a794c8d2c4807ad3207
created_at: 2026-10-17T22:08:17.024343243Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/mission_review.rs:644-651: a review or evidence record mentioned in record text is rendered with its ID as the visible label (the popover gives kind and summary). On atelier-r6bt the Work entries for sfsq, ho7o and rx22 read 'review atelier-tqgi showed...', 'Review atelier-dqty...', 'Review atelier-gq88 and atelier-yx83...', which the human reads as bare IDs, against Done when's 'no bare IDs in the HTML text' and the mockup, whose Changed lines name no IDs. Issue references are correct. Fix: label a record mention by its kind and a short summary (for example 'a finding' or the first words of its summary) and keep the ID only in the popover; add an assertion to the HTML fixture.
