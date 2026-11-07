---
created_at: "2026-11-07T05:01:00.802679334+00:00"
id: "atelier-md0v"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-md0v
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ilba"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Cut the records and gates that review does not need"
updated_at: "2026-11-07T08:34:54.029205352+00:00"
---

## Goal

Keep findings, decisions on deferred and dismissed findings, approvals and the merge gate; remove the bookkeeping and gates around them that the transcripts show cost turns and caught nothing.

## Done when

- [ ] A finding whose reviewer recommended `fix_now` and that has no disposition is resolved by an independent approval that is both pinned to a later commit and recorded after the finding (`created_at`), so an approval that never saw the finding cannot resolve it. A finding recommended `defer` or `no_action` still needs a recorded decision, even if it was fixed instead. Old `fix_now` dispositions, with or without a resolution commit, still count, and `review disposition --decision fix_now` stays accepted but is never asked for. Tests: finding, fix commit, approval, merge with no disposition command; and an approval recorded before a finding on an earlier commit does not resolve it.
- [ ] No gate reads evidence receipts: the built-in `evidence.no_newer_failure` gate and the `evidence.attached` validator are removed (a policy naming `evidence.attached` still parses and the name is ignored), and a fail receipt blocks neither merge nor land. Receipts remain as notes for what nothing else records.
- [ ] `land` and the Stop hook report and exit successfully while work is in progress on a branch; they refuse only what would put unreviewed code on the base. A hook run from a directory outside any Atelier repository reports that and exits successfully (the Stop half of atelier-gg0m; its SessionStart wording stays with gg0m). Tests for both.
- [ ] Removed: the rule that an approval's identity, or a `defer`/`no_action` decision's identity, differ from the work's Git authors (identities stay recorded); ADR 0025 decision 4 goes too, since atelier-ilba already accepts that a finding can be deleted before merge, so a non-author rule on decisions guards nothing. Also removed: the source rule on deferral follow-ups (`review_follow_up_source_mismatch`).
- [ ] The merge base is the `branch_policy.base_branch` of the branch's committed policy, checked against the base ref's own policy as merge already does; `land_base_mismatch` and every read of `branch_base` go (merge.rs, workflow_policy.rs `action_policy`, review_request.rs, git.rs, status.rs, session.rs). New ownership records keep writing `branch_base`, set to the policy base and ignored when read, so older pinned binaries still parse them, until atelier-ml3r moves astromech's CI pin and drops it.
- [ ] `start` uses no type prefix: on the base it creates a branch named with the bare item id; if the owner already has a recorded branch it switches to it as today; if the current branch is owned by another item that is not this item's parent it refuses and names that item; from a detached HEAD it refuses as today; otherwise it takes the current branch. `--base` is removed. Tests for each case.
- [ ] A unique short commit id is accepted wherever a command takes a commit (`--commit`, `--resolution-commit`, `review trivial --base`, evidence commands), resolved on input and stored as the full id; an ambiguous or unknown prefix is refused. Test.
- [ ] Merge, `land` on the base and `doctor` judge findings, decisions and approvals by the same rules: `findings.disposed`, `approval_accepted` (used by `land`'s base coverage) and doctor's historical paths (`original_independent`, `disposition_independent`, the `historical` branch of `evaluate_scope`) all apply the changes above.
- [ ] The lifecycle success criterion's scenario test extends crates/atelier-cli/tests/lifecycle_cost.rs: in a fixture repository with the `atelier init` starter policy, one issue goes through start; notes, links and ticked Done when; a review with one `fix_now` and one `defer` finding; a fix commit; re-review and approval; a note and a link edited after approval; merge; then `atelier land` on the base and `atelier doctor`. It finishes with no refusal and no approval beyond the fix's re-review, the issue is done in the merge commit, and the test prints how many `atelier` commands the author and reviewer ran (replacing the fixed count constants).
- [ ] ADR 0028 (decision 5, and the Supersession entries left open for this item: ADR 0016's review targets and mission integration branches, ADR 0025 decision 4), PRODUCT_INTENT.md and docs/product describe the rules as built.

## Scope

Gates in merge, land and doctor; the Stop hook path; `start`; review and evidence commands; workflow policy validators; tests; docs; ADR 0028. Must not change: merge refuses without a current independent approval of the code; a fix after review is re-reviewed (its approval is what resolves the finding). Other repositories keep working with the shared binary: old policies and records still parse.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 5), docs/strategies/simpler-lifecycle.md (Shape: Fewer records and gates), crates/atelier-app/src/review_gate.rs (`independent`, `findings.disposed`, `approval_accepted`), crates/atelier-cli/src/commands/start.rs and git.rs, crates/atelier-app/src/record_index.rs, crates/atelier-core/src/lib.rs (`WorkflowBranch`). atelier-jnrh (merge marks done) and atelier-ilba (write-time validation) land first. A branch with no issue is atelier-4nsl's light path; `start` taking the current branch must not stand in its way.
