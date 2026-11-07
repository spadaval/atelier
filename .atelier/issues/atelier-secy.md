---
created_at: "2026-11-07T03:35:59.047087245+00:00"
id: "atelier-secy"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-secy
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "atelier-htux"
    type: "discovered_from"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T03:43:52.492014755+00:00"
status: "done"
title: "Ticking an after-merge check on the base refuses land for reviewed items"
updated_at: "2026-11-07T03:43:52.492014755+00:00"
---

## Goal

The after-merge path atelier-n0qh built (ADR 0027) works for every merged item: ticking a `- [ ] After merge` line, or adding `carried to <id>`, in a records-only commit on the base passes `land`. Today it does only for items with no review findings. For an item that had a fixed `fix_now` finding, the edit to its done record makes `land` refuse with findings.disposed ("1 fix_now finding resolved after the current approval; it needs a fresh approval"), so the validation step the docs describe needs a fresh review, which defeats it.

## Done when

- A test: an item with a `fix_now` finding fixed and re-approved, merged, then its after-merge check ticked in a records-only commit on the base: `land` passes. Same for adding `carried to <existing id>`.
- Any other edit to the done record on the base is judged as today (a test shows a non-validation edit of the same record still refuses).
- The validation-only edit test reuses atelier-n0qh's rule (only the box and an added "carried to <id>" change; check text, reason and stand-in unchanged), not a second definition.
- The proof_timing scenario item has a fixed fix_now finding, so the scenario covers this case.

## Context

Found by the mission validation (receipt atelier-htux). The refusal is findings.disposed in crates/atelier-app/src/review_gate.rs (approval_accepted), reached from land in crates/atelier-cli/src/commands/session.rs. Reproduction: /home/supadava/.cache/atelier-tmp/n4mj-scripts/repro.sh atelier <dir> 1 (0 for the no-finding case that lands). The pre-mission binary refuses the same record edit, so the gate is not new; the after-merge path just never accounted for it. Strategy: docs/strategies/lighter-workflow.md (Shape "Proof has a time", "Show, don't enforce").

## Close Reason

Ticking an after-merge check, or adding `carried to <existing id>`, in a records-only commit on the base now passes `land` for an item whose fixed `fix_now` finding preceded its approval; any other edit of the merged done record still refuses with findings.disposed. The fix is in `completion_claim_matches` (crates/atelier-app/src/evidence_freshness.rs): a Done when that differs only as `after_merge::only_validated` allows is the same done claim, so land judges the item as a claim the base already holds, and `claim_endpoint` ends at the original claim instead of the tick.

Proof:
- `proof_timing::a_check_after_merge_is_briefed_shown_until_validated_and_refuses_nothing`: the scenario's item now has a `fix_now` finding, fixed and disposed before approval; the tick on the base lands. Passes; without the fix it fails at the post-tick `land` (findings.disposed).
- `proof_timing::carrying_a_check_on_the_base_lands_but_editing_what_done_means_does_not`: carrying the check to a new bug in one base commit lands (fails without the fix, same refusal); then rewording the before-merge Done when line refuses with findings.disposed (as before the fix).
- `/home/supadava/.cache/atelier-tmp/n4mj-scripts/repro.sh` with 1 and 0: both `land after tick: passed` (with 1 it was refused before).
- Mission criterion receipt atelier-fk5f (proof-timing, pass) re-recorded on this branch.
- `cargo nextest run -j 4 -p atelier-app` (114 passed) and `-p atelier-cli` (769 passed), `cargo fmt -- --check`, `git diff --check` clean; no new clippy warnings in changed files.

### Deviations
None.

### Decisions
- The rule lives in the done-claim comparison, not in approval freshness (`allowed_edge`), so a tick on a work branch after approval is still a change the approval did not see; only the base's judgement of an already-merged claim changes. `completion_claim_matches` now takes the record index (for `carried to` targets) and is directional (`before`, `after`); `claim_endpoint` had the arguments the other way round and now passes them oldest first.

### Surprises
- The same tick also marked the item's evidence receipts stale ("acceptance changed since the pin"): receipt staleness (`first_mismatch`) compared the whole Done when, so under an `evidence.attached` close gate `land` still refused. Review finding atelier-xqjc; fixed in c1bc3ac4 by the same rule (`same_acceptance`), proven by `proof_timing::a_receipt_still_proves_its_item_after_its_check_is_ticked_on_the_base` (passes; fails without the fix with evidence.attached "found 0").

### Open items
- Suite with `--features test-hooks` and macOS left to CI.
