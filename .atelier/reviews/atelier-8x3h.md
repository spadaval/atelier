---
schema: atelier.review
schema_version: 2
id: atelier-8x3h
title: 'union_preserving (crates/atelier-app/src/review_gate.rs:234-266) compares each version as a set of lines, so a resolution can move lines between relationship roles and drop the owner''s blocker with the approval kept (atelier-e2xv is still open in this form). Each entry is several lines, ''- kind: "issue"'' repeats, and ''id:'' lines have the same indent under blocks and relates. Reproduced: on the base, open issue Z blocks the owner (blockers.none_open FAILs). The branch links Z to Y as related before approval, and the base then links Z to X as related. The base merge conflicts only in Z''s record. The resolution keeps every line but puts id X under blocks and the owner''s id under relates. atelier check passes, and atelier merge --check passes blockers.none_open, blockers.transitive_none_open and review.current. The same line set can also drop an entry outright when its id line appears in another role, or move a line into the body. Fix: compare structure, not line sets. Parse the front matter of base, ours, theirs and the resolution, and require each relationship role''s set of (kind, id, type) entries and each scalar to equal the three-way merge of the two sides. Use a multiset only for body lines. Add the retarget case to base_merge_record_resolutions_keep_approval_only_when_they_lose_nothing.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: acffbe36af6bb949038f0313feedb2a5c15affd2
created_at: 2026-10-16T20:09:44.799559403Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: decb65a8e086ca314995b9534104ce64d67261eb
    recorded_at: 2026-10-16T20:21:13.197706152Z
    resolution_commit: 9c48a595e3a3a3799103820a7a2609c30488fd24
    reason: 'Resolved. Replayed by hand: label conflict on an out-of-scope record X that blocks Y; the resolution keeps both labels and moves Y from blocks to relates (type related). atelier check passes, merge --check FAILs review.current. Lists now compare whole entries, so the blocks list is short one entry. The control union resolution passes review.current.'
    migrated_from: atelier-mc93
---

union_preserving (crates/atelier-app/src/review_gate.rs:234-266) compares each version as a set of lines, so a resolution can move lines between relationship roles and drop the owner's blocker with the approval kept (atelier-e2xv is still open in this form). Each entry is several lines, '- kind: "issue"' repeats, and 'id:' lines have the same indent under blocks and relates. Reproduced: on the base, open issue Z blocks the owner (blockers.none_open FAILs). The branch links Z to Y as related before approval, and the base then links Z to X as related. The base merge conflicts only in Z's record. The resolution keeps every line but puts id X under blocks and the owner's id under relates. atelier check passes, and atelier merge --check passes blockers.none_open, blockers.transitive_none_open and review.current. The same line set can also drop an entry outright when its id line appears in another role, or move a line into the body. Fix: compare structure, not line sets. Parse the front matter of base, ours, theirs and the resolution, and require each relationship role's set of (kind, id, type) entries and each scalar to equal the three-way merge of the two sides. Use a multiset only for body lines. Add the retarget case to base_merge_record_resolutions_keep_approval_only_when_they_lose_nothing.
