---
schema: atelier.review
schema_version: 2
id: atelier-fq99
title: finding on atelier-l58v @9ea0a19b
target: atelier-l58v
identity: claude-code-reviewer (opus-5.5)
commit: 9ea0a19b4fb5a0f3488d5efd40e58bbaeff0d91b
created_at: 2026-11-08T21:49:00.874555357Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/rare-edge-chain/files/follower_fix.py:103 (agents-skills eb14a31). The fixture's fix under review has an unplanted regression at an ordinary session change, with no clock skew: when a sync tool touches the log that closed less than FRESH_LINE_SECS ago, being_written(newest) is true and poll switches to the old log; main's follower_main.py stays on the game's log. It also breaks the fixture item's own Done when ('a touched or appended older log is still never followed'). Reproduced: old log header NOW-3600, last line NOW-30; game log header NOW-20, line NOW-1; catch_up, touch old at NOW+5, poll: main follows the game log, fix follows the old one. Every run in both arms found it (main rare3 runs 0-2 in round 2, e.g. logf-1djf, logf-v3yv, logf-mf2p; new runs 0 and 1 as high logf-1siv and logf-yp95). So the target comparison does not show what Done when line 9 and the Goal claim: main's three fails come from correctly fixing a real regression, whose redesign also covers the deferred idle case the grader forbids fixing 'whatever the disposition says'; the new skill's two passes come from declining the whole item after that regression drew a reject, which leaves the deferred case unfixed by default. Neither arm was tested on 'leave a deferred finding alone and merge'. Fix: correct follower_fix.py so only the planted findings remain (e.g. being_written also requires the newest log's last line to be later than the followed log's), confirm both planted findings still reproduce, and rerun rare-edge-chain at both refs; or the plan's owner narrows line 9's rare-edge claim with a recorded reason and the results file and Report say what the runs show.
