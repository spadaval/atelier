---
schema: atelier.review
schema_version: 1
id: atelier-ctr1
title: 'land skips a done claim that a change to another issue record invalidates, so the induction does not hold. blockers.none_open, blockers.transitive_none_open and work.terminal read other records, and completion_claim_matches ignores relationships, so none of these counts as a change to the claim: (a) atelier issue link <done> <open> --role blocked_by writes only the blocker''s record; (b) atelier issue create --parent <done> adds an open child, and the parent''s children-list edit still matches the claim; (c) reopening a done blocker or child edits only that record. Reproduced on the base with an upstream: land passes, doctor reports ''blockers.none_open: open blockers'' or ''work.terminal: open work''. Master''s land refused these. Fix: run the index-only graph checks (blockers, work.terminal) for every retained claim, since they need no git history; or mark as touched any claim whose dependency closure, ancestors or descendants include an issue whose record changed since the trusted point. Also count any change to the claim''s own record (including parent_id, which sets the approval targets for FixNow acceptance), not only completion_claim_matches, and fix the ''its record changed'' wording in sessions-and-hooks.md.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6039144f2ace03f72f3112f5c4f01afa5cfce4f7
created_at: 2026-10-16T23:29:50.637152292Z
event:
  event: finding
  recommendation: fix_now
---

land skips a done claim that a change to another issue record invalidates, so the induction does not hold. blockers.none_open, blockers.transitive_none_open and work.terminal read other records, and completion_claim_matches ignores relationships, so none of these counts as a change to the claim: (a) atelier issue link <done> <open> --role blocked_by writes only the blocker's record; (b) atelier issue create --parent <done> adds an open child, and the parent's children-list edit still matches the claim; (c) reopening a done blocker or child edits only that record. Reproduced on the base with an upstream: land passes, doctor reports 'blockers.none_open: open blockers' or 'work.terminal: open work'. Master's land refused these. Fix: run the index-only graph checks (blockers, work.terminal) for every retained claim, since they need no git history; or mark as touched any claim whose dependency closure, ancestors or descendants include an issue whose record changed since the trusted point. Also count any change to the claim's own record (including parent_id, which sets the approval targets for FixNow acceptance), not only completion_claim_matches, and fix the 'its record changed' wording in sessions-and-hooks.md.
