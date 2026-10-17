---
schema: atelier.review
schema_version: 1
id: atelier-ulvb
title: 'Low: docs/adr/0019-verified-repo-native-tracker.md:74-76 still says land checks claims that a change ''since the branch base or the base''s upstream'' can affect; after this change an owned branch trusts only the merge base of the branch base with the base''s upstream (nothing without an upstream), so the ADR''s summary of the decision now contradicts the product doc. One-line edit, e.g. ''since the merge base with the base''s upstream''. Also docs/product/sessions-and-hooks.md:75 was left as an unwrapped ~115-char line when the paragraph was rewritten; rewrap.'
target: atelier-kops
identity: Independent reviewer (Claude subagent)
commit: b04debc463d854a7c0a5bae96eaee5459b306f81
created_at: 2026-10-17T06:04:04.673531824Z
event:
  event: finding
  recommendation: fix_now
---

Low: docs/adr/0019-verified-repo-native-tracker.md:74-76 still says land checks claims that a change 'since the branch base or the base's upstream' can affect; after this change an owned branch trusts only the merge base of the branch base with the base's upstream (nothing without an upstream), so the ADR's summary of the decision now contradicts the product doc. One-line edit, e.g. 'since the merge base with the base's upstream'. Also docs/product/sessions-and-hooks.md:75 was left as an unwrapped ~115-char line when the paragraph was rewritten; rewrap.
