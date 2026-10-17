---
schema: atelier.review
schema_version: 1
id: atelier-xbn7
title: 'crates/atelier-app/src/mission_review.rs:299-301: an item counts as delivering a criterion when it holds the deciding receipt, but deciding() is the newest receipt in history whatever its result. The item whose receipt fails a criterion is therefore shown as delivering it: the work meta reads ''Feature · <slug>'', and the row reads ''Failing ... by <item>''. Fix: credit by receipt only when that receipt passes the criterion.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:49.408385785Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/mission_review.rs:299-301: an item counts as delivering a criterion when it holds the deciding receipt, but deciding() is the newest receipt in history whatever its result. The item whose receipt fails a criterion is therefore shown as delivering it: the work meta reads 'Feature · <slug>', and the row reads 'Failing ... by <item>'. Fix: credit by receipt only when that receipt passes the criterion.
