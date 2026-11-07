---
schema: atelier.review
schema_version: 2
id: atelier-7hyh
title: 'crates/atelier-app/src/mission_review.rs:256-263: follow_up_source and follow_ups_of record every defer disposition, even when the finding''s latest disposition is something else. A finding deferred and later re-disposed fix_now is counted as ''1 fixed'' in the Review line, yet its follow-up still shows under ''Left open'' and in Open work ''from <item>''. Reproduced in a scratch repo. Fix: derive follow-ups from the latest disposition per finding, the same map the counts use.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:49.145251064Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    recorded_at: 2026-10-17T22:02:50.193287051Z
    resolution_commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    reason: Fixed in ff8838b691bc4425883a80bc6bdc4128f53579dd with tests; see the commit message
    migrated_from: atelier-a4yh
---

crates/atelier-app/src/mission_review.rs:256-263: follow_up_source and follow_ups_of record every defer disposition, even when the finding's latest disposition is something else. A finding deferred and later re-disposed fix_now is counted as '1 fixed' in the Review line, yet its follow-up still shows under 'Left open' and in Open work 'from <item>'. Reproduced in a scratch repo. Fix: derive follow-ups from the latest disposition per finding, the same map the counts use.
