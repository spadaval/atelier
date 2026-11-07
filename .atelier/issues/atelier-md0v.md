---
created_at: "2026-11-07T05:01:00.802679334+00:00"
id: "atelier-md0v"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Cut the records and gates that review does not need"
updated_at: "2026-11-07T05:01:00.802679334+00:00"
---

## Goal

Keep findings, decisions on deferred and dismissed findings, approvals and the merge gate; remove the bookkeeping and gates around them that the transcripts show cost turns and caught nothing.

## Done when

- [ ] The approval that follows a `fix_now` finding resolves it: no resolution commit or separate `fix_now` disposition is asked for. `defer` and `no_action` still record a decision.
- [ ] No gate reads evidence receipts: a fail receipt blocks no close, land or merge. Receipts remain as notes for checks nothing else records.
- [ ] `land` and the Stop hook report and exit successfully while work is in progress on a branch; they refuse only what would put unreviewed code on the base.
- [ ] Removed: the check of reviewer identity against Git authors (the identity stays recorded), the recorded base branch and branch-type rules, the source rule on deferral follow-ups (`review_follow_up_source_mismatch`), and full-length-only commit arguments (a unique short id is accepted).
- [ ] `scripts/merge-ci-passed.sh` treats strategy files as planning, not code.
- [ ] The lifecycle success criterion's scenario test passes with at most 8 `atelier` commands.

## Unknowns

- What replaces the recorded base for a branch that targets a mission branch rather than the base.
