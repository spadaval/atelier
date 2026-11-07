# ADR 0026: Evidence Records Only What Nothing Else Records

## Status

Accepted (2026-11-06, human-directed: Sumeet directed the lighter-workflow
mission, atelier-n4mj, to keep receipts only for checks that leave no other
record, and accepted that repositories without CI lose their merged-tree
check; atelier-s66u).

## Context

An audit on 2026-11-06 read all 1,725 receipts recorded since 2026-10-15 in
atelier, astromech, demiurge and cosplay-studio, classified a sample of 72 by
hand, and read every fail and blocked receipt and the agents' transcripts.

- About a quarter of the sample was useful and more than half was busywork:
  whole-suite, fmt and diff-check captures that CI or the merge checks already
  gate. One item got 18 receipts in 23 minutes.
- Base merges caused 47 re-records in atelier and 90 in astromech; 8 were
  needed because an earlier receipt had failed. `merge --sync` reran 28
  receipts in atelier, and 26 of them were never read. 49 merge refusals told
  agents to record fresh proof.
- No policy required a passing receipt to close or merge any more
  (`evidence.attached` left atelier's policy in 5c4a38ef). Agents recorded
  these receipts because merge and land errors, `status`, `work` and the docs
  told them to.
- The useful receipts were browser checks, runtime checks on a real build
  host, defects found in validation, tests that failed before their fix,
  measurements and mission criteria receipts. Reviewers rerun checks
  themselves and rarely relied on receipts.

## Decision

A receipt records a check that leaves no other record: a manual, browser,
device or deployed check, a mission criterion, a measurement, or a failure
(including a test that fails before its fix). Suites, fmt and lint that CI or
the policy's `merge_checks` run need no receipt; the item's Close Reason names
the targeted test or check that proves each claim. A repository without CI
puts its suite in `merge_checks`.

These rules are removed:

- **Asking for proof again.** Merge and land errors, the land behind-base
  note, the docs and Agent Factory's text no longer tell agents to record or
  re-record proof after review fixes, base merges or rewritten history.
- **No receipt as a gap.** `status` readiness no longer reads `no proof at
  head`, and `work parent` no longer counts `Proof gaps`. Both show only a
  failing receipt with no pass after it (`⚠ failing evidence <id>`,
  `Failing evidence`, `failing_evidence=`); an item with no receipts is clean.
  `work mission` keeps its criteria marks.
- **`merge --sync` reruns.** The sync no longer reruns command-backed
  receipts or commits new ones (nor its `sync_rerun_failed` and
  `sync_rerun_error` refusals); it merges the base in and reports the
  approval.
- **Freshness for clearing a failure.** A pass recorded after a failure
  clears it for good, even once later commits make that pass stale. Before,
  the pass cleared it only while fresh, so a base merge or a later commit
  revived the failure and asked for proof again. With it goes land's
  `Uncommitted changes make passing proof stale` note.

What stays: receipts and their freshness marks in `evidence list`, failure
output, criteria receipts and `work mission`'s marks, the
`evidence.no_newer_failure` block on close, land and merge, and a policy's
own `evidence.attached` gate, which still asks for a fresh pass. Command
capture keeps refusing uncommitted code (Sumeet, 2026-11-06), so a receipt
always describes committed state; that is what lets a later pass clear a
failure for good. Agent Factory's Core Rule "Prove claims with fresh
evidence" stands: a check is still run after the last change and its output
read; only whether it is recorded as a receipt changes.

## Consequences

- Repositories with neither CI nor `merge_checks` (demiurge and
  cosplay-studio today) have no merged-tree check once `merge --sync` stops
  rerunning receipts, until they add a suite merge check (atelier-bp3s).
  Sumeet accepted this on 2026-11-06.
- Fewer receipts and fewer record-only commits. Most of the saving is in how
  agents behave, which no scenario test shows; atelier-akkp re-measures real
  repositories after release.
- A failure fixed long ago no longer blocks after unrelated later work, and
  a fail recorded after a pass still blocks until a newer pass. A pass that
  later commits made stale is still marked stale in `evidence list`, so
  freshness stays visible where it is judged.
