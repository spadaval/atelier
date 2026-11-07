---
schema: atelier.review
schema_version: 2
id: atelier-n5wl
title: finding on atelier-bqbz @709ebff0
target: atelier-bqbz
identity: lighter-workflow reviewer for atelier-bqbz (claude-opus-5-5)
commit: 709ebff0842519d604ec8d2c3fcc9926aee49548
created_at: 2026-11-07T03:14:14.594734044Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: atelier-bqbz implementer (claude-opus-5-5)
    commit: f05fd995a809da873eef57c10ab6c4f8d091de77
    recorded_at: 2026-11-07T03:15:33.375717129Z
    resolution_commit: f05fd995a809da873eef57c10ab6c4f8d091de77
    reason: work-view-ordering.md says started work with a blocker stays active, shows its blocker count in work active, and close still refuses
---

low: docs/product/work-view-ordering.md:64 Started work that hits a blocker now stays in_progress, as the item decides. I checked this in a fixture with the branch binary: it shows under 'work active' as '| 1 blocker', it is absent from 'work blocked', and status's Current Work row shows no blocker mark. Started work also has no way back to backlog any more, since the in_progress -> block -> defer path is gone and only supersede remains. close still refuses while the blocker is open, so no gate is lost. The doc's 'blocked: work with one or more open blockers' line now misleads: classify_issue (work_classification.rs:244) ranks the active category before blockers. Fix later: say in work-view-ordering.md that active work with blockers stays active, or decide whether 'work blocked' should list it. Either way it is shown, not refused.
