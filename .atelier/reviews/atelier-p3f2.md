---
schema: atelier.review
schema_version: 2
id: atelier-p3f2
title: '.atelier/issues/atelier-v5tt.md Done when: the narrowing to atelier-7b6e is recorded honestly for walk batching. But the original clause ''the time between Git calls (about 2.2 s of 5.5 s, finding atelier-fvxd) is attributed and reduced or recorded as accepted'' was dropped without being carried into atelier-7b6e or recorded as accepted. The Design phase timers attribute time by gate phase, not the inter-call gap. Also, the new Done when says ''fewer serial Git calls'', but the record gives only wall times (7.3 s to 5.1 s, fixture unchanged within noise at load average 9, disclosed). It has no before/after Git process or serial-call count of the kind atelier-83xi measured (about 920 processes). Record the gap disposition and a call count, or state that they were not measured.'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 76a364da360c29b9afe0cdf075c9934ba73b4e5c
created_at: 2026-10-16T21:59:55.867094175Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 3841028c54e10d40fe6d109c806768be33cf0b2c
    recorded_at: 2026-10-16T22:08:59.840726604Z
    resolution_commit: bc1ae1c335dff65e1782a9cea9c7736d5d499f3f
    reason: 'Resolved: the v5tt Result now gives Git processes per land, 1,336 to 1,136, and names the method (GIT_TRACE2 start events). The inter-call-time clause is now in atelier-7b6e''s Done when (attributed and reduced or recorded as accepted), and v5tt''s Done when says it was split there. I did not re-measure the counts. They come from a different method and repository size than atelier-83xi''s strace count of 920, so they compare only with each other.'
    migrated_from: atelier-1feh
---

.atelier/issues/atelier-v5tt.md Done when: the narrowing to atelier-7b6e is recorded honestly for walk batching. But the original clause 'the time between Git calls (about 2.2 s of 5.5 s, finding atelier-fvxd) is attributed and reduced or recorded as accepted' was dropped without being carried into atelier-7b6e or recorded as accepted. The Design phase timers attribute time by gate phase, not the inter-call gap. Also, the new Done when says 'fewer serial Git calls', but the record gives only wall times (7.3 s to 5.1 s, fixture unchanged within noise at load average 9, disclosed). It has no before/after Git process or serial-call count of the kind atelier-83xi measured (about 920 processes). Record the gap disposition and a call count, or state that they were not measured.
