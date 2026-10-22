---
created_at: "2026-10-17T01:36:20.485223894+00:00"
id: "atelier-tciw"
issue_type: "feature"
labels:
- "cli-output"
- "status"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-tciw
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-cisa"
    type: "related"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "issue"
    id: "atelier-n642"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-w56o"
    type: "related"
  - kind: "issue"
    id: "atelier-ydv0"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show each waiting branch's merge readiness against the current base in status"
updated_at: "2026-10-22T22:24:17.488160179+00:00"
---

## Goal

An orchestrator running parallel work branches can't see which approved branches would merge cleanly into the current base, which would conflict and in which files, or whose approval is stale. It brings the base into every waiting branch after each landing instead. In astromech on 2026-10-16, feature/astromech-cmhv merged main into itself 16 times while it waited 4.5 hours to land. Eight of those merges were clean and needed fresh proof. Eight conflicted, in Cargo.lock, Cargo.toml, docs/ARCHITECTURE.md, crates/astromech-core/src/pipeline.rs and app/src/lib/core.ts, and each needed a fresh independent review: 18 review commits on one branch. None were needed until cmhv was next to land. Agents rebuilt the missing view by hand: `git merge-tree --write-tree … | grep CONFLICT` loops over branches, and `git merge-base --is-ancestor master HEAD` before each merge.

## Done when

- The branches section of `status` (atelier-ftqn) shows one dense line per owned work branch with a done claim, for example:
  `astromech-cmhv  approved  behind 3  merges cleanly  proof at branch head only`
  `astromech-sfys  approved  behind 3  conflicts: Cargo.lock, crates/astromech-core/Cargo.toml`
  `astromech-m30a  approval stale (merge 04b6f78, conflict resolution)  behind 0  ready`
- Each line covers four things: commits behind the base, whether `git merge-tree` against the current base is clean or which files conflict, approval state (current, stale with the cause from atelier-cisa, or none), and whether proof covers the branch head.
- A trailing line names waiting branches that change the same files, for example `Shared files: cmhv and sfys both touch Cargo.lock`, so the orchestrator can land them back to back.
- The view changes no Git state; it only runs `git merge-tree`. Tests cover the clean, conflicted, stale-approval and shared-file cases.

## Context

Blocked by atelier-ftqn, which rebuilds the same branches section. It uses the stale cause that atelier-cisa adds. It pairs with orchestrators updating a branch only when it is next to land, and with a merge strategy that merges a clean branch into the base. The work came out of the 2026-10-16 transcript retrospective; the figures come from astromech-atelier's history (`git log --merges --grep='Merge main into feature/astromech-cmhv'`, then `git show --remerge-diff`).

## Notes

- 2026-10-22 [note] tciw now also owns the approval mark on the Branches rows (moved from atelier-ftqn). It should expose a public end-commit variant of approval currency (today the private approval_is_current_to in review_gate.rs) rather than rebuilding it, and follow the rule atelier-n642 settles.
- 2026-10-22 [note] From review of atelier-ftqn: put readiness facts in the status part of the row, before the free-text title, as ftqn's 'done, not merged ✓ approved  Title' example does (finding atelier-465o). Also show activity on a branch whose owner is a parent, e.g. done or active children, so a parent-owned branch does not read as idle (finding atelier-p3dv).
- 2026-10-22 [decision] Readiness facts extend the row's comma-separated status part (approval, behind N, proof, merge state), with the conflict list last so its commas end the part
  Reason: Finding atelier-465o puts branch facts before the free-text title; a fixed order with conflicts last keeps the comma list unambiguous
  Rejected: Two-space-separated fields as in the Done when example, which read like the title separator
- 2026-10-22 [decision] Approval state comes from a new review_gate::approval_state that reuses review.current's rule (independence against author_identities, current_on_branch extracted from evaluate_scope, approval_is_current_to); an approval by a branch author counts as not approved, not stale
  Reason: One rule for merge and status; a non-independent approval would never become current, so calling it stale misleads
  Rejected: Marking approved when the branch snapshot holds an approval record
- 2026-10-22 [decision] 'proof at head' means every done item in the owner's scope, except superseded ones, has fresh passing evidence at the branch tip by the evidence freshness rule; otherwise 'no proof at head'
  Reason: merge's completion checks ask fresh proof of each done claim in scope; checking only the owner would call a parent-owned branch proven when its children are not
  Rejected: Proof of the owner alone
  Rejected: 'proof at branch head only' wording about the merged tree, which merge_checks prove instead
- 2026-10-22 [decision] Shared files compare each pair of waiting branches' paths changed since their fork point, records included, one line per pair naming up to five paths
  Reason: Shared records such as a mission record are real merge hazards; capping keeps a pair that shares many files to one readable line
  Rejected: Excluding .atelier paths
- 2026-10-22 [decision] Measured status cost and added no cache: with five waiting branches in a clone of this repository, release status went from 0.85 s to 1.15 s (about 60 ms per waiting branch); debug 2.25 s to 3.4 s
  Reason: The item asks to report cost rather than add caches; readiness is computed only for done owners that are not merged
