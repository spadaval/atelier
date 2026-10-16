---
schema: atelier.review
schema_version: 1
id: atelier-jpad
title: 'Reviewed git diff master...HEAD at b90f72c9 (fix acd89b0a). The Sequence arm of merged now requires equal length plus, for every resolved entry, the same count in resolved and expected. That is exact multiset equality: with equal lengths, matching counts for every value in resolved accounts for all of expected, so expected cannot hold an extra value. The multiplicity case expected [dup,dup,branch,base] resolved [dup,branch,base,base] (the [a,a,b]->[a,b,b] shape) is now refused; it was accepted by the earlier one-way-containment commit 7ab80627. Test list_resolutions_keep_both_sides_and_add_nothing_even_with_duplicates: the smuggled-entry assertion fails against master''s check (len 4 and all expected entries present), and the base,base assertion fails against both master and the intermediate commit, so the test catches both regressions. The correct union is still accepted and a dropped side is refused. Quadratic counting is fine for label/link-sized lists. The expected-list construction did not change. Ran TMPDIR=~/.cache/atelier-tmp cargo nextest run --workspace at head: 642/642 passed, matching evidence atelier-5z5h (commit acd89b0a, pass).'
target: atelier-wgil
identity: Independent reviewer (Claude subagent)
commit: b90f72c9f74a55607c1548c34535d3412b247e58
created_at: 2026-10-16T20:39:16.677506513Z
event:
  event: approval
---

Reviewed git diff master...HEAD at b90f72c9 (fix acd89b0a). The Sequence arm of merged now requires equal length plus, for every resolved entry, the same count in resolved and expected. That is exact multiset equality: with equal lengths, matching counts for every value in resolved accounts for all of expected, so expected cannot hold an extra value. The multiplicity case expected [dup,dup,branch,base] resolved [dup,branch,base,base] (the [a,a,b]->[a,b,b] shape) is now refused; it was accepted by the earlier one-way-containment commit 7ab80627. Test list_resolutions_keep_both_sides_and_add_nothing_even_with_duplicates: the smuggled-entry assertion fails against master's check (len 4 and all expected entries present), and the base,base assertion fails against both master and the intermediate commit, so the test catches both regressions. The correct union is still accepted and a dropped side is refused. Quadratic counting is fine for label/link-sized lists. The expected-list construction did not change. Ran TMPDIR=~/.cache/atelier-tmp cargo nextest run --workspace at head: 642/642 passed, matching evidence atelier-5z5h (commit acd89b0a, pass).
