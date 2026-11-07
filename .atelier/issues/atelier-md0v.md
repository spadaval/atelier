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
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Cut the records and gates that review does not need"
updated_at: "2026-11-07T07:43:21.941154885+00:00"
---

## Goal

Keep findings, decisions on deferred and dismissed findings, approvals and the merge gate; remove the bookkeeping and gates around them that the transcripts show cost turns and caught nothing.

## Done when

- [ ] The approval that follows a `fix_now` finding resolves it: `findings.disposed` accepts a `fix_now` finding with no disposition when an independent approval is pinned to a later commit, and nothing asks for a resolution commit or a separate `fix_now` disposition. `defer` and `no_action` still record a decision. Test: finding, fix commit, approval, merge, with no disposition command.
- [ ] No gate reads evidence receipts: `evidence.no_newer_failure` and `evidence.attached` are removed (old policies naming them still parse and the names are ignored), and a fail receipt blocks neither merge nor land. Receipts remain as notes for what nothing else records.
- [ ] `land` and the Stop hook report and exit successfully while work is in progress on a branch; they refuse only what would put unreviewed code on the base. A hook run from a directory outside any Atelier repository reports that and exits successfully (from atelier-gg0m). Tests for both.
- [ ] Removed: the rule that an approval's identity, or a `defer`/`no_action` decision's identity, differ from the work's Git authors (identities stay recorded; ADR 0025 decision 4 goes with the reviewer rule, as the same comparison); the recorded base branch (`branch_base`, `land_base_mismatch`), since merge integrates only into the policy's base branch anyway; type-prefixed branch names (`start` on the base creates a branch named after the item, and on any other branch takes that branch); and the source rule on deferral follow-ups (`review_follow_up_source_mismatch`). Old records carrying `branch_base` still parse.
- [ ] A unique short commit id is accepted wherever a commit is (a small cut within the orchestrator's authority).
- [ ] The lifecycle success criterion's scenario test exists and passes: in a fixture repository with the `atelier init` starter policy, one issue goes through start; notes, links and ticked Done when; a review with one `fix_now` and one `defer` finding; a fix commit; re-review and approval; a note and a link edited after approval; merge. It finishes with no refusal and no approval beyond the fix's re-review, the issue is done in the merge commit, and the test prints how many `atelier` commands the author and reviewer ran.
- [ ] ADR 0028's decision 5 paragraph, PRODUCT_INTENT.md and docs/product describe the rules as built.

## Scope

Gates in merge and land, the Stop hook path, `start`, review and evidence commands, workflow policy validators, tests, docs, ADR 0028. Must not change: merge refuses without a current independent approval of the code; a fix after review is re-reviewed (its approval is what resolves the finding). Other repositories keep working with the shared binary: old policies and records still parse.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 5), docs/strategies/simpler-lifecycle.md (Shape: Fewer records and gates), crates/atelier-app/src/review_gate.rs (`independent`, `findings.disposed`), crates/atelier-cli/src/commands/start.rs and git.rs (`branch_base`, `land_base_mismatch`), crates/atelier-app/src/record_index.rs (`review_follow_up_source_mismatch`). atelier-jnrh (merge marks done) and atelier-ilba (write-time validation) land first.
