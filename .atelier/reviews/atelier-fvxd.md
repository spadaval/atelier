---
schema: atelier.review
schema_version: 1
id: atelier-fvxd
title: 'The diagnosis at .atelier/issues/atelier-83xi.md:37 calls everything other than the log walks small, but on this repository about 2.2 s of the 5.5 s land wall time is not covered by any git child process. GIT_TRACE2_EVENT attribution with binary 0.2.0+b9cdced8 in /home/supadava/projects/atelier: log walks cover 2.55 s of wall time. The 611 sequential merge-base --is-ancestor and rev-list --parents -n 1 processes run only 0.7 s together, but they leave 707 small gaps of 1 to 3 ms each, which is spawn and in-process work between children, plus one 0.64 s in-process gap. atelier-v5tt (.atelier/issues/atelier-v5tt.md:21-25) targets only the log walks. Failure scenario: v5tt reaches its walk-count Done-when and land on this repository still takes about 3 s, still growing with retained approvals through the per-approval merge-base and rev-list chain. Suggest v5tt or a sibling item also records the sequential ancestry and edge queries as a cost that scales with records. Not blocking: the history fixture already exposes it.'
target: atelier-83xi
identity: Independent reviewer (Claude subagent)
commit: a32065d797cb83692ce9d2ac1c7478ac162cc6cd
created_at: 2026-10-16T19:08:42.162908627Z
event:
  event: finding
  recommendation: defer
---

The diagnosis at .atelier/issues/atelier-83xi.md:37 calls everything other than the log walks small, but on this repository about 2.2 s of the 5.5 s land wall time is not covered by any git child process. GIT_TRACE2_EVENT attribution with binary 0.2.0+b9cdced8 in /home/supadava/projects/atelier: log walks cover 2.55 s of wall time. The 611 sequential merge-base --is-ancestor and rev-list --parents -n 1 processes run only 0.7 s together, but they leave 707 small gaps of 1 to 3 ms each, which is spawn and in-process work between children, plus one 0.64 s in-process gap. atelier-v5tt (.atelier/issues/atelier-v5tt.md:21-25) targets only the log walks. Failure scenario: v5tt reaches its walk-count Done-when and land on this repository still takes about 3 s, still growing with retained approvals through the per-approval merge-base and rev-list chain. Suggest v5tt or a sibling item also records the sequential ancestry and edge queries as a cost that scales with records. Not blocking: the history fixture already exposes it.
