---
schema: atelier.review
schema_version: 1
id: atelier-b9gm
title: 'crates/atelier-app/src/record_index.rs:229: the second recovery, ''delete the link from the linking record''s relationships by hand'', does not work when the referencing record is a review. Reproduced: ''Issue <review-id> has review target reference to missing issue <id>'' plus ''<review-id> has reviews reference ...''. A review has no relationships block and its target field is required. The same goes for evidence whose only target is the pruned issue: removing that link leaves the evidence without a target. Restoring with git show does work in every case I tried. Fix: word the alternative as ''or remove the link, or the review/evidence record that targets it''. Also, the existing ensure_issue message calls the review an ''Issue'' (pre-existing).'
target: atelier-r7cd
identity: Independent reviewer (Claude subagent)
commit: 85972cb43f9213363053eabf17e9a29dd7569afa
created_at: 2026-10-16T01:37:08.894568022Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_index.rs:229: the second recovery, 'delete the link from the linking record's relationships by hand', does not work when the referencing record is a review. Reproduced: 'Issue <review-id> has review target reference to missing issue <id>' plus '<review-id> has reviews reference ...'. A review has no relationships block and its target field is required. The same goes for evidence whose only target is the pruned issue: removing that link leaves the evidence without a target. Restoring with git show does work in every case I tried. Fix: word the alternative as 'or remove the link, or the review/evidence record that targets it'. Also, the existing ensure_issue message calls the review an 'Issue' (pre-existing).
