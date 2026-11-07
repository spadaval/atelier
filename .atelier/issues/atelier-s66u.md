---
created_at: "2026-11-06T23:09:15.884675353+00:00"
id: "atelier-s66u"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-s66u
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5fus"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T00:23:20.632827304+00:00"
status: "done"
title: "Evidence records only what nothing else records"
updated_at: "2026-11-07T00:23:20.632827304+00:00"
---

## Goal

A receipt records a check that leaves no other record: a manual, browser, device or deployed check, a mission criterion, a measurement, or a failure (including a test that fails before its fix). Suites, fmt and lint that CI or the merge checks run need no receipt, and nothing in Atelier tells agents to record or re-record them.

## Done when

- no hint, doc or Agent Factory text asks agents to record or re-record proof after review fixes or base merges: at least the "record fresh proof" hints in crates/atelier-cli/src/commands/merge.rs and commands/session.rs (land's behind-base note and land_state_changed), docs/product/validation.md and reviews-and-merge.md, and Agent Factory's procedures/orchestrate.md "Landing Parallel Branches" step that reruns proof after a base merge; grep the code and both doc trees for "fresh proof" and "record proof" to find the rest;
- `status` and `work` views stop treating an item with no receipt as a proof gap: readiness's "no proof at head" (crates/atelier-cli/src/commands/git.rs) and proof_gap_count (commands/work.rs) show only failing receipts and mission criteria marks, with a test that an item with no receipts reads as clean;
- `merge --sync` no longer reruns or commits receipts (crates/atelier-cli/src/commands/merge_sync.rs), and its docs say CI or the merge checks test the merged tree where the repository has them (see atelier-bp3s);
- a later passing receipt clears an older failure even after later code commits make the pass stale (crates/atelier-app/src/evidence_freshness.rs, newest-failure check), with a test of an old fail, a pass, then a base merge, then close and merge succeeding;
- the docs say repositories without CI put their suite in the policy's merge checks;
- Agent Factory text in ~/projects/agents-skills (references/atelier.md, references/tracker-records.md Evidence) says which checks get a receipt and that the close reason names the targeted test; the SKILL.md Core Rule "Prove claims with fresh evidence" stays as it is, since checks are still run fresh and read;
- receipts, failure output, criteria receipts and `work mission` marks, and the newer-failure block keep working (existing tests pass);
- evidence capture keeps refusing uncommitted code (Sumeet, 2026-11-06), so a receipt always describes committed state;
- an ADR records the removed rules and says that repositories without CI or `merge_checks` (demiurge and cosplay-studio today) have no merged-tree check until they add one (atelier-bp3s).

## Context

Kept as one item rather than split (orchestrator, 2026-11-06): most of it removes code and text, so one branch and one review cost less than three. Strategy: docs/strategies/lighter-workflow.md (Shape, "Evidence records only what nothing else records").

## Background

An audit on 2026-11-06 of all 1,725 receipts recorded since 2026-10-15 in atelier, astromech, demiurge and cosplay-studio, with a hand-classified sample of 72, all 65 fail and blocked receipts, and the agents' transcripts:
- No policy requires a passing receipt to close or merge (evidence.attached was removed from atelier's policy in 5c4a38ef); only the newer-failure block remains (crates/atelier-cli/src/commands/workflow.rs, close; crates/atelier-app/src/review_gate.rs, merge).
- Sample: 17 useful (24%), 14 moderate (19%), 41 busywork (57%). Busywork is mostly whole-suite, fmt and diff-check captures already gated by CI or merge checks; atelier-ksx6 got 18 receipts in 23 minutes.
- Base merges caused 47 re-records in atelier and 90 in astromech; 8 were required by an earlier fail. `merge --sync` reran 28 receipts in atelier, 26 never read. 49 merge refusals told agents to record fresh proof.
- Useful: browser checks (cosplay-studio-epm4), runtime checks on a build host (cosplay-studio-9mx9), real defects found in validation (astromech-t33g), before-fix fails (astromech-veqs), measurements (atelier-8ud1 latency), mission criteria receipts.
- Reviewers rerun checks themselves, as the review brief instructs, and rarely rely on receipts.

## Close Reason

Receipts now record only what nothing else records. A pass recorded after a failure clears it for good, even after later commits make the pass stale. `status` readiness and `work parent` show only a failing receipt with no pass after it, so an item with no receipts reads as clean. `merge --sync` merges the base and reports the approval, and no longer reruns or commits receipts. Merge, land and rebase-recovery hints, the product docs and Agent Factory no longer ask agents to record proof again. ADR 0026 records the removed rules and the merged-tree gap in demiurge and cosplay-studio.

Proof, by Done when check (targeted tests, run fresh after the last change, all passing; per the new rule no receipts were recorded for them):
- Old fail, then pass, then base merge, then close and merge succeed: `merge_gates::a_later_pass_clears_an_older_fail_across_a_base_merge_so_close_and_merge_succeed` (pass). It fails under the old rule, which `verified_records::newer_failure_blocks_completion_without_presence_gate_until_any_later_pass` (formerly `..._stale_pass_cannot_clear_it`) asserted the opposite of. Also `records_evidence::evidence_list_verdict_agrees_with_the_close_gate` and `session::a_current_claim_whose_pass_a_later_commit_staled_keeps_its_failure_cleared` and `a_retained_claim_..._keeps_its_failure_cleared` (pass).
- No receipts reads as clean: `session::waiting_branches_show_merge_readiness_against_the_base` (a branch with no receipts shows no proof fact; a fail shows `⚠ failing evidence <id>`; a later pass clears it) and `issues::test_work_parent_failing_evidence_counts_its_scope_not_the_parent` (0 with no receipts, 2 with fails on both children, 1 after a pass) (pass).
- `merge --sync` records no receipts: `merge_gates::sync_merges_the_base_records_no_receipts_and_reports_the_approval_kept` (head is the base merge itself, one command receipt as before) and `sync_stops_at_conflicts_with_nothing_committed_and_resumes_after_the_resolution` (pass).
- Hints: `merge_gates::merge_check_on_a_diverged_base_also_reports_the_review_and_claim_gates`, the rebase recovery test and `session` rewrite tests assert the reworded hints (pass). `grep -rniE "fresh proof|record proof"` over crates, docs and the Agent Factory skill finds only the `evidence` command's help line, tests of a policy's `evidence.attached` gate, the June audit doc, and Agent Factory's "run fresh proof" (running, not recording).
- Receipts, failure output, criteria marks and the newer-failure block keep working: the existing evidence, criteria and gate tests pass. Capture's refusal of uncommitted code is unchanged.
- `cargo nextest run --workspace` (960 passed), the same for `atelier-app` and `atelier-cli` with `--features test-hooks` (864 passed), `cargo fmt -- --check` and `git diff --check` are clean.

### Deviations
- The Done when named only merge.rs and session.rs hints. Rewording the rebase-recovery hints in `review_gate.rs` ("record fresh proof and review" became "get them reviewed again") and the `evidence.attached` hint in `workflow_validation.rs` ("Record fresh proof" became "This policy requires a passing receipt: record one with ...") also came from the grep. The `review_gate.rs` edits are small and kept away from the findings code atelier-hwxm changes: the newer-failure gate block, the `GateNode.stale_from_checkout` field, and two hint strings.
- Removed land's "Uncommitted changes make passing proof stale" note and the newer-failure reasons that named a stale later pass (atelier-749l, atelier-x3bj). Once any later pass clears a failure, nothing is left for them to explain.
- `merge --sync` no longer prints "Resuming after a merge of <base> made by hand". With nothing to rerun, a second sync after a hand merge just says there is nothing to merge. It still uses the hand merge's first parent to say whether the approval was current before.
- Quiet token `proof_gaps=` became `failing_evidence=`, and `work parent`'s `Proof gaps` row became `Failing evidence`.

### Decisions
- Any pass recorded after a fail clears it, whatever its pin or freshness: one rule, with no new conditions on when it applies (Shape: "simplicity wins"). `newest_failure` is now a pure function over the record index, with no Git reads.
- A policy's own `evidence.attached` gate still needs a fresh pass. It is opt-in, and the item does not remove it.
- The ADR is 0026: 0024 was taken, and atelier-hwxm took 0025 (renumbered by the orchestrator before review).

### Surprises
- Manual `evidence record` (no command) does not refuse uncommitted code. Only capture does. The Done when only says capture keeps refusing, so this is unchanged. A manual pass recorded over dirty code now clears an older fail too.

### Open items
- ADR number: settled as 0026; atelier-hwxm has 0025.
- The Agent Factory edits are on agents-skills branch feature/atelier-s66u (commit a4462c3, worktree ~/projects/agents-skills-wt/atelier-s66u), committed but not pushed or reviewed.
- Independent review of both branches is outstanding.
