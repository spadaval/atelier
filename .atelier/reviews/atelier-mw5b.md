---
schema: atelier.review
schema_version: 2
id: atelier-mw5b
title: 'G13: Local Git replacement refs reinterpret a recorded resolution and approval pin, allowing stale approval to pass while HEAD, canonical records, identities and full recorded object IDs remain unchanged. Independent reproduction changes only the replacement view. Proof, approval and immutable history checks must use the original committed objects; replacement views cannot redefine what the recorded pin verified.'
target: atelier-1ioo
identity: session-reviewer
commit: 6f5188dd5bc990c88733a0027ee703eaaef9f737
created_at: 2026-10-02T00:59:29.059656263Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator
    commit: f331bf0bdff04007ef8cc3fc37e9402bc02b2eb8
    recorded_at: 2026-10-02T01:13:42.134937533Z
    resolution_commit: f331bf0bdff04007ef8cc3fc37e9402bc02b2eb8
    reason: 'G13 independently verified in root composition: proof and review checks use original commit objects under default and custom replacement namespaces; legitimate history and pruning pass, dirty owner protection and stale approval refuse.'
    migrated_from: atelier-3ksg
---

G13: Local Git replacement refs reinterpret a recorded resolution and approval pin, allowing stale approval to pass while HEAD, canonical records, identities and full recorded object IDs remain unchanged. Independent reproduction changes only the replacement view. Proof, approval and immutable history checks must use the original committed objects; replacement views cannot redefine what the recorded pin verified.
