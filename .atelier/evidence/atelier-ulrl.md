---
created_at: "2026-11-07T15:34:14.725746962+00:00"
id: "atelier-ulrl"
evidence_type: "validation"
commit: "72d05b0303f8a1f547c9e994ad6438ee44902f0c"
result: "fail"
captured_at: "2026-11-07T15:34:14.725702309+00:00"
agent_identity: "independent validator (claude-opus-5-5), atelier-2bho"
target:
  kind: "issue"
  id: "atelier-mmpv"
  role: "validates"
criteria:
- slug: "every-repo"
  result: "fail"
criteria_mission: "atelier-mmpv"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mmpv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "every-repo partly holds. Policy: the base-branch workflow.yaml of atelier (master), astromech (origin/main e6ae38df), cosplay-studio (main), demiurge (master) and life-on-rails (main) has schema_version 3 and merge_commit, no close transition on the work workflow (close stays only on the mission's objective workflow), and an on_base workflow for loose_end and validation. Check and doctor: with the installed atelier 0.3.1+135c1f9e, `atelier check` and `atelier doctor` exit 0 in all five checkouts as they are, and on a scratch clone of astromech main. The astromech checkout is on feature/astromech-v6r3, 163 commits behind main, and still has the old policy (fast_forward_only, blocked and review statuses, a work close transition), so it passes under the old policy, not the new one. After merge: `git grep -c \"After merge (\"` finds nothing on cosplay-studio, demiurge or life-on-rails. On atelier master it finds 2 lines in atelier-n0qh (lines 49 and 59), both prose about ADR 0027, so no open check. On astromech origin/main 4 open lines remain: astromech-1hh9:67, astromech-v3u4:85 (not in the `After merge (<reason>):` form, so `atelier check` misses it), astromech-v6r3:40 and astromech-y6d6:55. Each belongs to an item in flight on another branch (astromech-vucf, astromech-y6d6, feature/astromech-v6r3) that astromech-zk7z left to migrate its own line. `atelier check` on main notes 3 of them. astromech-dl53:67 tells the author to add an After merge line if a spike fails, and astromech-zk7z:70 is prose. On in-flight branches: astromech-y6d6 also has dl53:65, and feature/astromech-v6r3 has 7f11:34, 7yb5:29, icde:33 and v6r3:45. Only astromech-ok29's line became a loose end (astromech-3fpi). Branches not merged into atelier, cosplay-studio, demiurge or life-on-rails have none. Limits: read-only; demiurge and life-on-rails bases are local, hundreds of commits ahead of origin."
updated_at: "2026-11-07T15:34:14.840401479+00:00"
---

every-repo partly holds. Policy: the base-branch workflow.yaml of atelier (master), astromech (origin/main e6ae38df), cosplay-studio (main), demiurge (master) and life-on-rails (main) has schema_version 3 and merge_commit, no close transition on the work workflow (close stays only on the mission's objective workflow), and an on_base workflow for loose_end and validation. Check and doctor: with the installed atelier 0.3.1+135c1f9e, `atelier check` and `atelier doctor` exit 0 in all five checkouts as they are, and on a scratch clone of astromech main. The astromech checkout is on feature/astromech-v6r3, 163 commits behind main, and still has the old policy (fast_forward_only, blocked and review statuses, a work close transition), so it passes under the old policy, not the new one. After merge: `git grep -c "After merge ("` finds nothing on cosplay-studio, demiurge or life-on-rails. On atelier master it finds 2 lines in atelier-n0qh (lines 49 and 59), both prose about ADR 0027, so no open check. On astromech origin/main 4 open lines remain: astromech-1hh9:67, astromech-v3u4:85 (not in the `After merge (<reason>):` form, so `atelier check` misses it), astromech-v6r3:40 and astromech-y6d6:55. Each belongs to an item in flight on another branch (astromech-vucf, astromech-y6d6, feature/astromech-v6r3) that astromech-zk7z left to migrate its own line. `atelier check` on main notes 3 of them. astromech-dl53:67 tells the author to add an After merge line if a spike fails, and astromech-zk7z:70 is prose. On in-flight branches: astromech-y6d6 also has dl53:65, and feature/astromech-v6r3 has 7f11:34, 7yb5:29, icde:33 and v6r3:45. Only astromech-ok29's line became a loose end (astromech-3fpi). Branches not merged into atelier, cosplay-studio, demiurge or life-on-rails have none. Limits: read-only; demiurge and life-on-rails bases are local, hundreds of commits ahead of origin.
