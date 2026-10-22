---
schema: atelier.review
schema_version: 1
id: atelier-jjol
title: 'medium: docs/product/human-cli-output.md:66-69,132-133; docs/product/sessions-and-hooks.md:131-134; docs/product/work-model.md:206-209 now contradict the shipped Branches rows. human-cli-output says the status other-branch rows use the Item Line (<id> <status mark> <priority> <title> | cue) and drop a ''| N active'' cue; sessions-and-hooks says a done owner''s item line ends ''| not merged into <base>'' / ''| merged into <base>''; work-model says other-branch rows show ''✓ done on branch <b>, not merged into <base>''. The branch emits ''<branch>  <id> <status>[, merged|, not merged]  <title>'' and an ''Unowned:'' line, and touches no doc. Impact: the output contract that tests and future views (atelier-tciw) are judged against describes a format that no longer exists. Fix: describe the Branches section row format and the Unowned line in human-cli-output.md (as a stated exception to Item Line, or by extending Item Line), and update the other two passages.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.435857612Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/product/human-cli-output.md:66-69,132-133; docs/product/sessions-and-hooks.md:131-134; docs/product/work-model.md:206-209 now contradict the shipped Branches rows. human-cli-output says the status other-branch rows use the Item Line (<id> <status mark> <priority> <title> | cue) and drop a '| N active' cue; sessions-and-hooks says a done owner's item line ends '| not merged into <base>' / '| merged into <base>'; work-model says other-branch rows show '✓ done on branch <b>, not merged into <base>'. The branch emits '<branch>  <id> <status>[, merged|, not merged]  <title>' and an 'Unowned:' line, and touches no doc. Impact: the output contract that tests and future views (atelier-tciw) are judged against describes a format that no longer exists. Fix: describe the Branches section row format and the Unowned line in human-cli-output.md (as a stated exception to Item Line, or by extending Item Line), and update the other two passages.
