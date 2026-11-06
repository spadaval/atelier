---
created_at: "2026-11-06T21:26:18.549920057+00:00"
id: "atelier-8s1m"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-abdp"
    type: "related"
  - kind: "issue"
    id: "atelier-efnf"
    type: "related"
  - kind: "issue"
    id: "atelier-hwxm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jzh9"
    type: "related"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A fix re-check counts as the review of that fix"
updated_at: "2026-11-06T22:08:52.543267636+00:00"
---

## Goal

An independent reviewer confirming a fix_now at or after its resolution commit has reviewed that fix. A review that raises fix_now findings may approve its pinned commit on condition that they are fixed; a review without that verdict is not an approval. A conditional approval plus a confirmation of every fix_now counts as approval of the pinned commit plus the resolution commits. A fresh approval of the whole head is needed only for commits no confirmed resolution covers, or for a finding the reviewer marks as needing re-review.

## Done when

Provisional: in the lifecycle scenario, with the review giving a conditional approval, a fixed finding needs no approval of the head beyond the reviewer's confirmation; an author-recorded fix_now with no independent confirmation is still refused; a review that raised findings without a conditional approval is not an approval, even once every fix_now is confirmed; a confirmation by an author identity is refused by the same non-author check as approval (and atelier-jzh9's implementer check, if it has landed); a commit beyond the confirmed resolutions still needs a fresh approval; a finding marked for re-review still requires a fresh approval; approval-freshness docs state the rule in fewer words than today; Agent Factory review text in ~/projects/agents-skills says the re-check satisfies its "changes after review" rule; an ADR records the replaced rule.

## Unknowns

Whether scoping review to the fix lets problems through; judged from findings during the mission, and if so taken to Sumeet. Whether a confirmation pinned at a resolution commit counts as a new review round for atelier-abdp.
