---
created_at: "2026-11-07T05:00:04.971587750+00:00"
id: "atelier-mmpv"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/simpler-lifecycle.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Take one issue from start to merge in one obvious order, where only a code change makes an approval stale"
updated_at: "2026-11-07T16:43:10.237195632+00:00"
---

## Outcome

An agent takes one issue from start to merge in one obvious order: start, implement, tick every Done when line, review, fix and re-review, approve, merge. Only a code change makes an approval stale, merge marks the issue done, post-merge work is a loose end outside the mission, records are checked when written and correctable by their author until they reach the base, and only the records and gates review needs remain. Independent review before merge, and re-review of fixes, are unchanged. The strategy, docs/strategies/simpler-lifecycle.md, owns the outcome, success criteria, shape and authority.

## Notes

- 2026-11-07 [handoff] 2026-11-07: all eleven work items are merged (3jdm, 4yqd, ilba, jnrh, 8bh4, md0v, 4nsl, l457, ml3r, nn67, uqxh), the skill rewrite is on spadaval/.agents main (4d88ffb), and all five repositories run the new policy. Criteria: lifecycle pass (atelier-oh52; 12 author + 3 reviewer commands), legible pass on re-check (atelier-l6jx, after uqxh cleared the four contradictions atelier-pznw found), every-repo partial (atelier-ulrl): policy, check and doctor hold everywhere, but four After merge lines remain on astromech items in flight on other threads' branches (1hh9, v3u4, v6r3, y6d6), which migrate when those branches merge. Open for Sumeet: the every-repo carve-out and decision atelier-mzpn. Follow-ups outside the mission: atelier-hfme, atelier-f4xz, atelier-xz5c, astromech-vfsu.
