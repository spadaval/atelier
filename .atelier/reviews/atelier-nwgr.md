---
schema: atelier.review
schema_version: 2
id: atelier-nwgr
title: finding on atelier-qr5s @f2782b3d
target: atelier-qr5s
identity: gpt-6-astra-plan-r3
commit: f2782b3d53b87a0475c95a5cd7def08f1be34049
created_at: 2026-11-10T01:32:23.198769400Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: e957f11c464ab4c1e511edbbc2f1c966897d8354
    recorded_at: 2026-11-10T01:40:06.791513365Z
    follow_up: atelier-u1ko
    reason: 'Deferred to the switch step''s planning boundary: atelier-u1ko''s Unknowns now names it.'
---

medium atelier-u1ko: step 5 updates other repositories' Atelier CI pins, but the transition names only an old-tool release and a local install of the new tool. A CI runner cannot use that local installation. At step E's planning boundary, choose and publish a reproducible schema-3 binary source before updating those pins, and exercise a representative pinned CI installation; dist-workspace.toml and release.yml publish artifacts from version tags.
