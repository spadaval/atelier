---
schema: atelier.review
schema_version: 1
id: atelier-3e17
title: finding on atelier-ll3p @298bd742
target: atelier-ll3p
identity: independent planning reviewer subagent (claude-opus-5-5)
commit: 298bd742a940e9add66c3b95a579760fb26c7e47
created_at: 2026-10-30T06:49:35.333741562Z
event:
  event: finding
  recommendation: defer
---

R6: Criterion review-history-shown and atelier-ma4a's Goal still say a Done when edited 'after a finding or failing receipt', while atelier-r9hb now triggers on any edit descending from the item's start commit and names a finding only when there is one. Not wrong (the new trigger is a superset), but the receipt judging the criterion needs r9hb's tests to include the after-a-finding case explicitly. Fix when convenient: reword the criterion and ma4a's Goal to 'edited after the item started', or make r9hb's Done when name the with-finding test case.
