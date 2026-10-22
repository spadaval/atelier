---
created_at: "2026-10-17T19:00:36.347834302+00:00"
id: "atelier-iaas"
issue_type: "bug"
labels:
- "merge"
- "retro-2026-10-17"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T22:20:00.399924466+00:00"
status: "done"
title: "Let a follow-up be linked to its mission without staling the branch approval"
updated_at: "2026-10-22T22:20:00.399924466+00:00"
---

## Goal

Linking a follow-up filed on an approved branch to its mission stales the approval (5rs0, o3lm 06:31Z). Linking it from the base fails with 'Issue atelier-jx7g was not found' because the issue exists only on the branch (07:18Z). There is no clean way to file a follow-up during review.

## Done when

Either creating a new issue record and its links on an approved branch leaves the approval current (the review gate ignores records that are new and not the item's own), or a base-side link may name an issue on an unmerged branch. `docs/product` describes the rule; tests cover filing a follow-up on an approved branch and merging it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). The memory rule 'link follow-ups on base' records the workaround.

## Notes

- 2026-10-22 [decision] Take the route that matches atelier-n642: an issue outside the approved scope that is absent at the approval's pin is planning in every later commit, so it may be created, linked to its mission (stored on the follow-up since atelier-ihss) and edited after approval while the approval stays current. allowed_edge now asks whether the record existed at the pin rather than at the edge's parent.
  Reason: On master after atelier-1ztv a follow-up created, given --discovered-from and linked in one commit already kept the approval (allowed_edge's new-record rule); the gap was a link or edit in a later commit, which saw an existing out-of-scope record and staled the approval. Its own done claim is still refused by claims.reviewed and land, since it is outside every approval's scope.
  Rejected: Let a base-side link name an issue that exists only on an unmerged branch (needs cross-branch resolution and breaks base record validity)

## Close Reason

An issue outside the approved scope that did not exist at the approval's pin is planning in every later commit, so a follow-up filed on an approved branch with `--discovered-from`, then linked to its mission in a later commit, leaves the approval current and the branch merges and lands under both merge strategies; closing that follow-up on the branch still fails `claims.reviewed` and `land`. `docs/product/reviews-and-merge.md` (Approval freshness) states the rule.

### Deviations
The branch also resolves the deferred finding atelier-p5qp from atelier-1ztv in the same commit (bbd0f996): docs and doc comments now describe that `land` leaves out every merged side branch's authors where `review.current` counts a side branch not on the base, with a comment at original_independent's call site. Doc and comment change only.

### Decisions
See the decision note on atelier-iaas (route: records new since the pin, not base-side links to unmerged issues) and on atelier-r28p (p5qp wording).

### Surprises
On master after atelier-1ztv the single-commit case already held (allowed_edge accepted any new out-of-scope record); only a link or edit in a later commit staled the approval. `atelier issue link <mission> <issue> --role advances` writes the mission's record, which still stales the approval; the documented direction is `link <follow-up> <mission>`.

### Open items
None.
