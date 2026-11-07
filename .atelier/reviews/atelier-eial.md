---
schema: atelier.review
schema_version: 2
id: atelier-eial
title: finding on atelier-hwxm @b6903e91
target: atelier-hwxm
identity: lighter-workflow reviewer for atelier-hwxm (claude-opus-5-5)
commit: b6903e91654d68c5664e05ef2c1951302fe6f538
created_at: 2026-11-07T01:03:20.011952296Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-app/src/review_gate.rs:348 review_change_allowed accepts removing any retired disposition event, folded or not. A pre-upgrade branch whose history deletes a finding's latest event (a reviewer's fix_now superseding an earlier no_action), which the old append-only check refused, passes after migrate: the earlier event becomes latest and is folded. It needs a deliberate deletion on a branch the old gate already blocked, so it is narrow. Possible fix: accept a retired event's removal only in a commit where its finding's disposition is migrated from that event's chain head (the event no other supersedes), or otherwise show it in the review brief.
