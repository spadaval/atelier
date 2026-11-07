---
schema: atelier.review
schema_version: 2
id: atelier-8poo
title: 'crates/atelier-app/src/mission_review.rs:452: planned_total counts every linked item not filed by a deferred finding, including items added after the mission started. On atelier-r6bt the sidebar reads ''Planned work done 7 of 14'', but the approved mockup reads ''Increments done 7 of 7''. The 14 includes atelier-nexd itself, the S7 decision atelier-9y66 and the three validation follow-ups (0a52, juyb, mg7w), none of which were planned. The label states something the records contradict: added_after_start is already known. Fix: also filter out added (r6bt would then read 7 of 9), or count what the mockup counted, and assert the meter in a fixture.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:48.605832635Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    recorded_at: 2026-10-17T22:02:48.644106182Z
    resolution_commit: ff8838b691bc4425883a80bc6bdc4128f53579dd
    reason: Fixed in ff8838b691bc4425883a80bc6bdc4128f53579dd with tests; see the commit message
    migrated_from: atelier-7c86
---

crates/atelier-app/src/mission_review.rs:452: planned_total counts every linked item not filed by a deferred finding, including items added after the mission started. On atelier-r6bt the sidebar reads 'Planned work done 7 of 14', but the approved mockup reads 'Increments done 7 of 7'. The 14 includes atelier-nexd itself, the S7 decision atelier-9y66 and the three validation follow-ups (0a52, juyb, mg7w), none of which were planned. The label states something the records contradict: added_after_start is already known. Fix: also filter out added (r6bt would then read 7 of 9), or count what the mockup counted, and assert the meter in a fixture.
