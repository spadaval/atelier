---
created_at: "2026-10-22T19:58:07.454492914+00:00"
id: "atelier-1ztv"
issue_type: "feature"
labels:
- "land"
- "merge"
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-1ztv
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-as7i"
  - kind: "issue"
    id: "atelier-n642"
  - kind: "issue"
    id: "atelier-uw4x"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:30:00.479089843+00:00"
status: "done"
title: "Make land and merge agree on what review covers"
updated_at: "2026-10-22T21:58:40.714811649+00:00"
---

## Goal

`land` and `merge` apply one rule for what an approval covers. A record-only close commit keeps the approval current. Filing a follow-up on an approved branch keeps it too. Merge refuses, before integrating, any done claim that land would refuse. A planning commit that adds an issue to a mission re-checks no old done claims. A review record created inside a merge commit is found.

## Done when

- Each child's Done when holds, shown by its tests, with one independent review per child or per batch of children.
- `atelier doctor` passes on brewer-corp/astromech at 649dc8ec (atelier-as7i).
- Delivers atelier-k8h8 S7 (atelier-n642, atelier-iaas, atelier-ss25) and the gate-agreement part of S11 (atelier-uw4x, atelier-as7i).

## Unknowns

atelier-iaas offers two routes. Prefer the one that matches atelier-n642: records new on the branch that are not the item's own leave the approval current. Record the choice as a decision note.

## Background

The children all change the review gate: `crates/atelier-app/src/review_gate.rs` (introduced and scope queries, approval currency), `crates/atelier-cli/src/commands/git.rs` (`land_base_done_claim`) and `touched_claims` in `crates/atelier-cli/src/commands/session.rs`. They share one branch and run in order:
1. atelier-as7i first, because astromech's land and doctor are broken today.
2. atelier-n642 and atelier-uw4x as one rule. Once a close-only commit stays covered, land and merge accept it the same way, and merge refuses every other done claim land would refuse. State the rule once in `docs/product/`.
3. atelier-iaas.
4. atelier-ss25.
atelier-7b6e (batched history walks) follows atelier-as7i on its own branch.

## Notes

- 2026-10-22 [decision] merge applies land's base claim check itself (claims.reviewed gate, git::first_unreviewed_claim) on the commit it would put on the base, with the base head as the trusted point, instead of tightening allowed_edge.
  Reason: allowed_edge rightly keeps the approval current for a new out-of-scope issue (planning, atelier-cafy); the gap was that merge never asked land's coverage question. Running the same function on the would-be base tip makes the two verdicts agree by construction under both strategies.
  Rejected: Make a new done issue stale the approval in allowed_edge; copy the coverage check into merge
- 2026-10-22 [decision] land counts an approval as covering only if review_gate::approval_accepted holds: independent by original_independent (the history-based identity check merge's claim gates already use) and every finding on the scope pinned at or before the approval disposed by then (FIX NOW resolved at or before the pin, DEFER/NO ACTION accepted by a non-author).
  Reason: Matches what merge requires of an approval while judging from history, which is all land on the base has. Checked against this repository: land with the new binary in a scratch clone, upstream set 20, 60, 150, 300 and 500 first-parent commits back, gives the same verdict and the same count of commits outside review as the installed binary; the main checkout has no unpushed commits and passes.
  Rejected: Reuse evaluate_scope's candidate filter, which needs the branch's author set at merge time
- 2026-10-22 [decision] When the branch merges a side branch that is not on the base, land leaves that side's authors out of the independence check while review.current counts them, so merge can refuse where land would accept, never the reverse
  Reason: Merge refusing earlier is the safe direction; aligning it fully would need review.current to resolve side-branch provenance it does not track today
  Rejected: Make review.current skip side-branch authors too

## Close Reason

### Outcome
`land` and `merge` apply one rule for what an approval covers, from shared code. A review record or issue first added by a merge commit is found, and an untraceable disposition fails its gate by name instead of aborting land and doctor (atelier-as7i). An approval stays current across a commit that changes only the scope's own issue records by status and Close Reason, under both land and merge (atelier-n642). Merge refuses, before integrating, any done claim land would refuse, through the new `claims.reviewed` gate, and land counts an approval only when it is independent and its scope's findings are disposed (atelier-uw4x, review findings atelier-vz2p and atelier-n90q). Delivers the close-commit part of atelier-k8h8 S7 and the gate-agreement part of S11.

### Deviations
atelier-iaas (follow-ups on an approved branch) and atelier-ss25 (land's touched-claims scope) were taken out of this increment on 2026-10-22 so this rule reaches the base sooner: atelier-tciw, atelier-s2ei, atelier-7b6e and atelier-j8kg all build on it. They run next, on their own branch. `atelier doctor` on astromech at 649dc8ec now runs to completion but fails findings.disposed on a real gap in astromech's records (atelier-as7i's Close Reason).

### Decisions
See the decision notes on atelier-as7i, atelier-n642, atelier-uw4x and atelier-1ztv (claims.reviewed instead of staling in allowed_edge; land's approval-validity rule).

### Surprises
Review found that merge still integrated a new already-done out-of-scope issue that land then refused (atelier-vz2p), and that land accepted self-approvals (atelier-n90q). Both are fixed and tested under both merge strategies.

### Open items
An owner closed with no approval fails both claims.reviewed and review.current, which is redundant. Adjacent history-query gaps are noted on atelier-7b6e. astromech needs an independent approval of the astromech-y296 resolutions.
