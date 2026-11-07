---
schema: atelier.review
schema_version: 2
id: atelier-iukz
title: '.atelier/issues/atelier-ermg.md:41: the decision picks ''size:s|m|l|xl'', but the adopter that prompted it (astromech-atelier/.atelier/issues) already has 132 labels in the ''size-*'' form (64 size-s, 51 size-m, 13 size-l, 4 size-xl). Nothing breaks: ''check'' ignores ''size-*'', so existing repos still pass. But those labels get no display or validation, and the decision does not say how to convert them. Failure: astromech keeps unvalidated ''size-*'' labels, or someone adds ''size:'' alongside them, which gives two sizes per record that check cannot see. Fix: add a line to the decision on converting ''size-x'' to ''size:x'' (a one-time sed, or a small ''atelier migrate'' step), and file a follow-up for astromech.'
target: atelier-b7hb
identity: Independent reviewer (Claude subagent)
commit: 316cc70ef3a1117026263abb3c68de001aba148f
created_at: 2026-10-16T01:36:43.205966390Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 9f26e3b8d2675960c51d317ea3023dc8be2ca9ed
    recorded_at: 2026-10-16T01:40:53.142105477Z
    resolution_commit: 5f8d55702ca0c0bd94d5f6a80941f5222757e2c6
    reason: Fixed in 5f8d55702ca0c0bd94d5f6a80941f5222757e2c6 by adopting the size- form astromech already uses, so no conversion is needed
    migrated_from: atelier-xrp3
---

.atelier/issues/atelier-ermg.md:41: the decision picks 'size:s|m|l|xl', but the adopter that prompted it (astromech-atelier/.atelier/issues) already has 132 labels in the 'size-*' form (64 size-s, 51 size-m, 13 size-l, 4 size-xl). Nothing breaks: 'check' ignores 'size-*', so existing repos still pass. But those labels get no display or validation, and the decision does not say how to convert them. Failure: astromech keeps unvalidated 'size-*' labels, or someone adds 'size:' alongside them, which gives two sizes per record that check cannot see. Fix: add a line to the decision on converting 'size-x' to 'size:x' (a one-time sed, or a small 'atelier migrate' step), and file a follow-up for astromech.
