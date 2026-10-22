---
schema: atelier.review
schema_version: 1
id: atelier-bpfx
title: 'low: docs/product/work-model.md:206-210 still says status ''in its owner row and its Branches rows'' shows a done owner as ''✓ done on branch <work-branch>, not merged into <base>''. That is the owner-row form; Branches rows print ''<branch>  <id> ✓ done, not merged  <title>'' (tests/session.rs:4270, 4357; human-cli-output.md#branches). This is the unresolved third part of atelier-jjol: f0b16783 added a link but kept the wrong wording, so the two product docs now disagree on the row an agent reads. Fix: say the owner row shows ''✓ done on branch <work-branch>, not merged into <base>'' and the Branches row ''✓ done, not merged'' (both ''✓ done, merged into <base>'' once merged), or drop the Branches mention and keep only the link. While there, restore the blank line removed at docs/product/human-cli-output.md:145 so the Counts paragraph (''A list of counts leaves out its zeros'') stays its own paragraph.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: ce188e134bf80ab4ed03dfd2e30562d2b714ee23
created_at: 2026-10-22T20:49:02.466975783Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/product/work-model.md:206-210 still says status 'in its owner row and its Branches rows' shows a done owner as '✓ done on branch <work-branch>, not merged into <base>'. That is the owner-row form; Branches rows print '<branch>  <id> ✓ done, not merged  <title>' (tests/session.rs:4270, 4357; human-cli-output.md#branches). This is the unresolved third part of atelier-jjol: f0b16783 added a link but kept the wrong wording, so the two product docs now disagree on the row an agent reads. Fix: say the owner row shows '✓ done on branch <work-branch>, not merged into <base>' and the Branches row '✓ done, not merged' (both '✓ done, merged into <base>' once merged), or drop the Branches mention and keep only the link. While there, restore the blank line removed at docs/product/human-cli-output.md:145 so the Counts paragraph ('A list of counts leaves out its zeros') stays its own paragraph.
