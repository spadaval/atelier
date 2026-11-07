---
schema: atelier.review
schema_version: 2
id: atelier-3mbw
title: 'status on an unborn HEAD (right after init) prints ''Checkout: unavailable - git ls-tree -rz HEAD failed'' instead of the old ''No commits yet'' view.'
target: atelier-sdb0
identity: Independent reviewer (Claude subagent)
commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
created_at: 2026-10-17T02:41:20.425575957Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
    recorded_at: 2026-10-17T02:41:21.641180865Z
    follow_up: atelier-6tmk
    reason: Unborn-HEAD handling is atelier-6tmk's scope; widened there to cover status.
    migrated_from: atelier-r6av
---

status on an unborn HEAD (right after init) prints 'Checkout: unavailable - git ls-tree -rz HEAD failed' instead of the old 'No commits yet' view.
