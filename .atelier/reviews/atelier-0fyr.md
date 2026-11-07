---
schema: atelier.review
schema_version: 2
id: atelier-0fyr
title: finding on atelier-nmvv @963c75a7
target: atelier-nmvv
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:34.320568291Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner/ll3p (claude-opus-5-5)
    commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    recorded_at: 2026-10-31T00:11:36.352801374Z
    resolution_commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    reason: Fixed in the planning revision; g2h9 is settled by the re-review's approval of atelier-80qd
    migrated_from: atelier-2pra
---

R8: two small inaccuracies. The Goal command `evidence record --target <id> -- false` exits 2 because --kind is required; the rest of the recorded behaviour holds and a manual receipt without a summary does error ("requires a summary"). Design cites PRODUCT_INTENT.md for "trusts agents and makes shortcuts visible", but that file holds neither "trust" nor "shortcut"; its actual sentence is the one jzh9 quotes. Fix: add --kind test, and cite the human decision or the actual sentence.
