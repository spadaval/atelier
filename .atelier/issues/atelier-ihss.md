---
created_at: "2026-10-17T01:36:21.184833581+00:00"
id: "atelier-ihss"
issue_type: "feature"
labels:
- "schema"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ihss
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dmh5"
    type: "related"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T06:59:11.541248183+00:00"
status: "done"
title: "Store each link only on the record that declares it"
updated_at: "2026-10-17T06:59:11.541248183+00:00"
---

## Goal

Linking writes to the shared record:
- `atelier issue link X M --role advances` writes the edge into both X and M.
- `--role blocked_by` writes `blocks: X` into Y, the blocker.

So every branch that adds work advancing one mission, or blocked by one shared blocker, edits that record, and parallel branches conflict on its relationships list. On 2026-10-16 git reported 16 conflicts in `.atelier` files, mostly additive relationship lists on records that many items link to, such as astromech-610m. Agents kept a `/tmp/orch-scripts/union-relates.py` resolver for these conflicts. A hand resolution produced the duplicate-`id:` entry that atelier-tqru later caught. The same storage is why a `blocked_by` link to an approved owner stales its approval (docs/product/reviews-and-merge.md), and why atelier-o3lm's filing avoided a `blocked_by` link to atelier-bw0b.

## Done when

- New links are written only to the record that declares them: the X in `atelier issue link X Y`.
- Reverse directions are derived at read time for `issue show`, work views, `check` and gates. These include Blocking, and the work that advances a mission.
- Records with today's storage, two-sided or on the blocker's side, still read correctly. The item decides whether to migrate them.
- Linking new work to a mission or a shared blocker leaves that record unchanged, shown by a test.
- Gates that read edges give the same answers as before, shown by tests. These are blockers, mission scope, and in-scope record freshness for approvals.

## Context

atelier-dmh5 changes the same storage, recording the direction of directional context links, so the two should be sequenced. From the 2026-10-16 transcript retrospective.

## Close Reason

New links are written only to the declaring record (issue link X Y writes X; --role blocks writes the blocker's own blocks). Reverse directions are derived in the index for issue show, work views, check, blocker gates, mission scope, approval scope and land's claim re-check; legacy two-sided and blocker-side storage read the same and are not migrated. Tests: record_store_links_only_the_declaring_record and links_leave_the_shared_mission_and_blocker_unchanged_and_read_from_either_side (shared records byte-for-byte unchanged), declared_legacy_and_two_sided_link_storage_read_the_same (9 storage combinations), links_declared_by_new_work_keep_approval_and_edits_to_existing_records_stale_it. Full suite with test hooks passes at ee3e97f8 after merging master.
