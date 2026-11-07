---
created_at: "2026-11-07T05:01:00.583737629+00:00"
id: "atelier-ilba"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ilba
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T07:13:27.190040693+00:00"
status: "done"
title: "Refuse an invalid record when it is written, and let authors fix their own"
updated_at: "2026-11-07T07:38:58.983169314+00:00"
---

## Goal

Today a record that breaks a graph rule is written without complaint, then every later command refuses, and append-only rules stop its author fixing it. On 2026-11-05 this stalled astromech for 5 hours and ended in a force-push of main; cosplay-studio lost 8 dispositions and demiurge needed a throwaway issue. Validate every write against the rules `atelier check` applies, and let review records be corrected until they reach the base.

## Done when

- [x] Every command that writes records (including `start`, `merge`, `migrate` and `prune`) validates the records it writes, together with the records they link to, against the rules `atelier check` applies, and refuses before writing anything. Tests cover rules that survive this mission, such as a finding with competing disposition events (crates/atelier-records/src/review.rs) and a link to a record that exists only on another branch. (The deferral source rule, `review_follow_up_source_mismatch`, is removed by atelier-md0v and needs no write-time version.)
- [x] A record that points at an issue existing only on another branch is refused at write, and the message names that branch.
- [x] A review record can be edited or deleted until it reaches the base. Each edit is judged against the base as it stood where the edit joined it: for a commit that reached the base through a merge, whether the path existed at that merge's first parent; for an unmerged commit, whether it exists at the tip of the policy's base branch. Tests: write a finding on a branch, edit it in a later commit, delete another, merge; `atelier check`, `merge --check`, `land` and `doctor` all pass, on the branch and on the base afterwards. Editing a review record already on the base is still refused.
- [x] ADR 0028's decision 4 paragraph and PRODUCT_INTENT.md's "Guard valid records ..." and "Keep review and evidence append-only ..." principles describe the rule as built, including the risk noted below.
- [x] Removed, and listed in the implementer report: append-only enforcement for review records not yet on the base.

## Scope

Every record-writing command; the append-only check in `land`, `doctor` and merge gates; tests, docs/product, ADR 0028, PRODUCT_INTENT.md. Must not change: append-only for review records already on the base; the record formats (no migration needed). Evidence has no append-only check today and gets none.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 4), crates/atelier-app/src/review_gate.rs (`review_change_allowed` and the append-only check), crates/atelier-app/src/git_freshness.rs (`edges`), crates/atelier-cli/src/commands/session.rs (`land`).

## Design

- "On the base" is judged per edit, as in Done when, so a correction made on a branch and then merged is not refused afterwards; one `git cat-file` per record and edge is cheap enough.
- No identity rule: before a record reaches the base, any agent on the branch may correct it. Identity is self-declared, so an author check would add a rule without stopping anyone. Risk, accepted and noted in the ADR (planning review finding R13): an implementer could delete a reviewer's finding before merge, so no decision on it is recorded; it cannot let unreviewed code merge, since approvals still cover the code.

## Notes

- 2026-11-07 [decision] A write is refused only for the check findings it adds: the guard compares atelier check's findings on the records the write would leave with those on the records before it
  Reason: A write is never refused for a problem it did not make, and a write that fixes one passes; it also catches graph errors that name no record, such as a cycle
  Rejected: Requiring the written records and the records they link to to pass check: refuses fixes in a repository that already has a problem, and needs ID matching in messages that cycle errors do not carry
- 2026-11-07 [decision] The guard is one hook the CLI installs and RecordStore runs for every write, with RecordStore::batch holding a command's several writes so they are checked and written together; start, migrate epics and prune call it themselves, and merge checks its tip's records against the base's
  Reason: Every write path goes through the store, so a writer added later, such as atelier-jnrh's done marking in merge, is checked without remembering to call anything
  Rejected: An explicit call at each command: a later writer can forget it
  Rejected: One implicit batch per command flushed when it ends: commands reread records from disk and commit them with --commit-records before they end
- 2026-11-07 [decision] Under fast_forward_only, and when the base branch cannot be read, each review record change is judged at its commit's own first parent, so a review record stays final once committed
  Reason: History does not show where a fast-forward began, so after the fast-forward land and doctor could only judge a branch's correction as a direct commit to the base; judging it against the base tip before merge would accept it and then refuse it on the base, the trap this item removes. atelier-jnrh makes every merge a merge commit, which ends the exception
  Rejected: The base tip for unmerged commits under fast_forward_only: accepted, then refused after the fast-forward
- 2026-11-07 [decision] A corrected approval counts as it reads now, its identity judged at the oldest commit since which its file has held these bytes
  Reason: merge's review.current already reads an approval as it is; requiring it unchanged since recorded, as land's claim checks did, made the two disagree once approvals could be corrected, refusing on the base what merge accepted
  Rejected: Keeping approvals append-only before the base: an exception to the rule
- 2026-11-07 [decision] review_follow_up_source_mismatch is left out of the write-time guard, as the orchestrator directed; atelier check still reports it
  Reason: atelier-md0v removes the rule, so it gets no write-time version
- 2026-11-07 [resolution] Review finding atelier-oyh0 reversed one decision: approvals and trivial exemptions stay append-only from their first commit (fix b9f2214cf4063d8982c068e2ab45176f469f0b43). The earlier decision note on corrected approvals, and the Close Reason's mentions of corrected approvals (the original_independent deviation and its surprise), describe the reverted behaviour.

## Close Reason

Every record write now passes the rules `atelier check` applies before any file changes, and a write whose records would add a check finding is refused with `record_invalid`, writing nothing. The record store runs the guard for every write and checks a command's several writes together; `start`, `migrate epics` and `prune` call it themselves, `bundle apply` checks its staged records as one batch, and `merge` checks the records it puts on the base. A reference to a missing record, and a full ID that only another branch holds, name that branch. Review records can be edited or deleted until they reach the base, each change judged against the base where it joined; records on the base stay append-only apart from a finding's disposition.

Proof, run after the last change (each passes):
- Done when 1: `atelier-app` `record_guard::tests` (a write that gives a finding competing disposition events is refused; a write is refused only for the findings it adds); `atelier-records` `tests::a_batch_writes_every_change_or_none`; `atelier-cli` `write_guard` tests `a_write_that_check_would_refuse_is_refused_and_nothing_is_written` (linking a blocker to a mission in progress, which the installed binary writes and then every command refuses), `migrate_refuses_a_record_that_links_an_issue_on_another_branch` and `merge_refuses_records_that_would_fail_check_on_the_base`.
- Done when 2: `write_guard::a_link_to_an_issue_on_another_branch_is_refused_naming_the_branch` (`issue link`, `review disposition --follow-up`, and `check` after a hand edit), plus the migrate test above, which names the branch from the guard.
- Done when 3: `merge_gates::review_records_can_be_corrected_until_they_reach_the_base` under `merge_commit`: a finding edited and another deleted on the branch, and the approval corrected; `check`, `land`, `doctor` and `merge --check` pass on the branch and on the base after the merge; an edit on the base, and a deletion on an unmerged branch of a finding the base holds, are refused. Seen failing with the new rule disabled. The `fast_forward_only` append-only tests in `merge_gates` and `session` still refuse.
- Done when 4: ADR 0028 decision 4, PRODUCT_INTENT.md's two principles, and docs/product (reviews-and-merge, sessions-and-hooks, validation, retention-and-prune, cli-surface), docs/architecture and CONTEXT.md.
- Done when 5: Deviations below.
- `cargo nextest run -j 4`: 995 passed; with `--features test-hooks`: 1001 passed. `cargo fmt -- --check`, `git diff --check`, `target/debug/atelier check` pass. The new binary's `atelier check` passes in astromech, cosplay-studio, demiurge and life-on-rails (read-only), and `doctor` in scratch clones of all five repositories gives the installed binary's result.

### Deviations
- Removed: append-only enforcement for review records not yet on the base. `review_gate::append_only` lets an edit or deletion through when the base did not hold the record where the change joined it; `land`'s uncommitted-records advice commits such a correction instead of restoring it; and an approval no longer has to be unchanged since it was recorded to count for done claims.
- Under `fast_forward_only` (astromech, cosplay-studio, demiurge), and when the base branch cannot be read, each change is judged at its commit's own first parent, so a review record stays final once committed, as before. A commit on the base's first-parent line is judged at its own first parent, a case the Done when does not name. atelier-jnrh's merge commits end the `fast_forward_only` exception.
- Beyond the listed scope: a corrected approval is judged at the commit that last set it (`original_independent`), so merge's `review.current` and the claim checks on the base agree. `atelier check` and every load error also name the branch that holds a missing record, and so does a not-found for a full ID.
- `review_follow_up_source_mismatch` is left out of the write-time guard, as directed, since atelier-md0v removes it.
- `merge` leaves a workflow policy that does not parse to its own `workflow_config_parse_error` check rather than refusing it again as a record finding.

### Decisions
Recorded as decision notes on this item (2026-11-07): refuse only the findings a write adds; one guard in the record store with batches, rather than a call per command; judge every commit at its own first parent under `fast_forward_only`; judge a corrected approval at the commit that last set it; leave the deferral source rule out of the guard.

### Surprises
- A live accepted-then-refused path: `issue link <mission in progress> <open issue> --role blocked_by` was written, after which every command failed with `workflow_mission_dependency_bypass` (reproduced with the installed binary). It is now refused.
- `merge`'s `review.current` read an approval by its current bytes, while `land`'s claim checks required it unchanged since recorded. That was harmless while records were append-only and diverged once they could be corrected.
- Each write costs one more `atelier check` pass (two when the records it leaves have findings): about 0.6 s in a debug build on this repository's 2,000 records, about one release `check` (0.3 s).

### Open items
- Review is outstanding; the orchestrator arranges it.
- atelier-jnrh overlap: crates/atelier-cli/src/commands/merge.rs has two small changes (`append_only` takes the base's branch policy; `record_guard::check_merge` runs once the tip is known). `ReviewBase` reads `MergeStrategy::FastForwardOnly`; if atelier-jnrh removes that variant, drop that case. If atelier-jnrh writes the done status through `RecordStore` into the merged checkout, the guard checks it with no further change; any other write path should call `record_guard::check_changes`.
- `start` and `prune` have no dedicated refusal test. Their writes pass the guard, and every existing start and prune test runs through it, but their own checks leave no ordinary way to build a write the guard would refuse.
- cosplay-studio's `atelier doctor` fails with "review needs YAML front matter" under both the installed and the new binary. That predates this change and is not addressed here.
- No after-merge checks.
