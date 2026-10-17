---
schema: atelier.review
schema_version: 1
id: atelier-lceo
title: 'crates/atelier-app/src/mission_review.rs:480-492: the ask-folding rule hides an unproven criterion from Needs your review whenever an open needs-human or decision item''s Goal first sentence contains the slug as a whole word. Semantic slugs are often ordinary words: the fixture''s own slug ''fast'' is folded by the Goal ''Show fast to the human.''. A needs-human item that merely uses the word therefore removes an unproven criterion from the asks and from the sidebar count. The same word matching drives delivered_by. Fix: fold only when the Goal names the slug in a criterion-reference form (an S<n> or slug token next to a strategy or criteria mention), or at least document the limit. The Success criteria row still shows Unproven.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:49.675786630Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/mission_review.rs:480-492: the ask-folding rule hides an unproven criterion from Needs your review whenever an open needs-human or decision item's Goal first sentence contains the slug as a whole word. Semantic slugs are often ordinary words: the fixture's own slug 'fast' is folded by the Goal 'Show fast to the human.'. A needs-human item that merely uses the word therefore removes an unproven criterion from the asks and from the sidebar count. The same word matching drives delivered_by. Fix: fold only when the Goal names the slug in a criterion-reference form (an S<n> or slug token next to a strategy or criteria mention), or at least document the limit. The Success criteria row still shows Unproven.
