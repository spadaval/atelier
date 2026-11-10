---
schema: atelier.review
schema_version: 2
id: atelier-om80
title: finding on atelier-fcb7 @436ef6e8
target: atelier-fcb7
identity: gpt-6-astra-plan-r2
commit: 436ef6e89c7dcf1596e149190900c7201583d61d
created_at: 2026-11-10T01:18:21.191582734Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-pz1y (atelier-i31q still open in the item contract): the specification now permits base land only for missions/items paths, but this item still exempts all of .atelier/. Its stated proof allows a direct config.toml commit to bypass independent review and change future merge checks. Narrow the provisional criterion to the two directories and explicitly test rejection of a direct base config change.
