---
schema: atelier.review
schema_version: 2
id: atelier-xbn7
title: 'crates/atelier-app/src/mission_review.rs:299-301: an item counts as delivering a criterion when it holds the deciding receipt, but deciding() is the newest receipt in history whatever its result. The item whose receipt fails a criterion is therefore shown as delivering it: the work meta reads ''Feature · <slug>'', and the row reads ''Failing ... by <item>''. Fix: credit by receipt only when that receipt passes the criterion.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:49.408385785Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    recorded_at: 2026-10-17T22:02:50.717677667Z
    resolution_commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    reason: Fixed in ff8838b691bc4425883a80bc6bdc4128f53579dd with tests; see the commit message
    migrated_from: atelier-opr8
---

crates/atelier-app/src/mission_review.rs:299-301: an item counts as delivering a criterion when it holds the deciding receipt, but deciding() is the newest receipt in history whatever its result. The item whose receipt fails a criterion is therefore shown as delivering it: the work meta reads 'Feature · <slug>', and the row reads 'Failing ... by <item>'. Fix: credit by receipt only when that receipt passes the criterion.
