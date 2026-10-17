---
schema: atelier.review
schema_version: 1
id: atelier-o858
title: '[medium] procedures/orchestrate.md, new paragraph after step 5: ''a branch that merges cleanly skips steps 1 to 4'' includes step 3 (rerun its proof), but Atelier only requires one shell.check in merge_checks, and the documented example is ''git diff --check HEAD^ HEAD'', which proves nothing about behavior. A textually clean merge can still break semantically (base renames a function the branch newly calls); under this text an orchestrator skips the proof rerun and the merged tree lands checked only by whatever merge_checks happen to contain. ''builds'' also reads as ''compiles'', which Atelier does not do; it constructs the merge commit. Constitution: evidence should match the claim. Fix: condition the skip on the merge checks running the proof that step 3 would rerun (for example ''When the integration step runs the proof that matters on the merged tree ...''), or keep step 3 out of the skipped steps, and say ''creates'' rather than ''builds''.'
target: atelier-zqee
identity: Independent reviewer (Claude subagent)
commit: dcb2107de131a531bb41298e5edd137d643dcbc5
created_at: 2026-10-17T07:19:15.080256848Z
event:
  event: finding
  recommendation: fix_now
---

[medium] procedures/orchestrate.md, new paragraph after step 5: 'a branch that merges cleanly skips steps 1 to 4' includes step 3 (rerun its proof), but Atelier only requires one shell.check in merge_checks, and the documented example is 'git diff --check HEAD^ HEAD', which proves nothing about behavior. A textually clean merge can still break semantically (base renames a function the branch newly calls); under this text an orchestrator skips the proof rerun and the merged tree lands checked only by whatever merge_checks happen to contain. 'builds' also reads as 'compiles', which Atelier does not do; it constructs the merge commit. Constitution: evidence should match the claim. Fix: condition the skip on the merge checks running the proof that step 3 would rerun (for example 'When the integration step runs the proof that matters on the merged tree ...'), or keep step 3 out of the skipped steps, and say 'creates' rather than 'builds'.
