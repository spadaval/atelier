---
schema: atelier.review
schema_version: 1
id: atelier-q708
title: 'Low: crates/atelier-app/src/mission_review.rs:874-880 and crates/atelier-cli/src/commands/mission_review.rs:1480 Validation''s findings total sums reviews on every scoped target (open, dropped and children too) while the approval denominator counts delivered items only, so ''Item review: 0 of 1 approved | 9 findings'' can count findings on undelivered work as if they belonged to the delivered items. The HTML also says ''approved by an independent reviewer'' where the Markdown says ''approved'' and the code does not check independence. Fix: sum findings over WorkEntry.review, or label the total as covering all scoped work; align the wording.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:27.933107208Z
event:
  event: finding
  recommendation: defer
---

Low: crates/atelier-app/src/mission_review.rs:874-880 and crates/atelier-cli/src/commands/mission_review.rs:1480 Validation's findings total sums reviews on every scoped target (open, dropped and children too) while the approval denominator counts delivered items only, so 'Item review: 0 of 1 approved | 9 findings' can count findings on undelivered work as if they belonged to the delivered items. The HTML also says 'approved by an independent reviewer' where the Markdown says 'approved' and the code does not check independence. Fix: sum findings over WorkEntry.review, or label the total as covering all scoped work; align the wording.
