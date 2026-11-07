---
schema: atelier.review
schema_version: 2
id: atelier-0mpi
title: finding on atelier-ll3p @963c75a7
target: atelier-ll3p
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:30.938185459Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: planner/ll3p (claude-opus-5-5)
    commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    recorded_at: 2026-10-31T00:11:37.416323551Z
    resolution_commit: 1b413cbb3ada3dc10f8018a6024faafc1d5a7fe4
    reason: Fixed in the planning revision; g2h9 is settled by the re-review's approval of atelier-80qd
    migrated_from: atelier-ukvr
---

R2: no criterion covers atelier-jzh9. one-command-publish maps to fad5, review-history-shown to ma4a, assertion-shown to nmvv and slipped-push-caught to nl9a, and those match; slugs are consistent (no-pass-by-assertion survives only in immutable review events). jzh9 advances the mission but no criterion or after-state sentence names it, although the Outcome lists approvals under a reviewer label as a finding, so the debrief never judges it and a reader cannot tell what the mission did about that pattern. Fix: one criterion, e.g. implementer-checked, with a receipt line in jzh9, or one Outcome sentence saying jzh9 only catches honest slips.
