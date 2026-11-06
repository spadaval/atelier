---
created_at: "2026-11-06T23:09:15.884675353+00:00"
id: "atelier-s66u"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Evidence records only what nothing else records"
updated_at: "2026-11-06T23:09:15.884675353+00:00"
---

## Goal

A receipt records a check that leaves no other record: a manual, browser, device or deployed check, a mission criterion, a measurement, or a failure (including a test that fails before its fix). Suites, fmt and lint that CI or the merge checks run need no receipt, and nothing in Atelier tells agents to record or re-record them.

## Done when

Provisional:
- merge errors (crates/atelier-cli/src/commands/merge.rs, the "record fresh proof" hints), the `status` "no proof at head" line (crates/atelier-cli/src/commands/git.rs, readiness) and docs/product/validation.md and reviews-and-merge.md no longer ask for fresh proof after review fixes or base merges;
- `merge --sync` no longer reruns or commits receipts (crates/atelier-cli/src/commands/merge_sync.rs), and its docs say CI or the merge checks test the merged tree;
- a later passing receipt clears an older failure even after later code commits make the pass stale (crates/atelier-app/src/evidence_freshness.rs, newest-failure check), with a test of an old fail, a pass, then a base merge, then close and merge succeeding;
- the docs say repositories without CI put their suite in the policy's merge checks;
- Agent Factory text in ~/projects/agents-skills (SKILL.md proof rule, references/atelier.md, references/tracker-records.md Evidence) says which checks get a receipt and that the close reason names the targeted test;
- receipts, failure output, criteria receipts and `work mission` marks, and the newer-failure block keep working (existing tests pass);
- an ADR records the removed rules.

## Unknowns

Whether capture should keep refusing uncommitted code (the run, commit, run-again pattern) once suite receipts are gone; decided during the item.

## Background

An audit on 2026-11-06 of all 1,725 receipts recorded since 2026-10-15 in atelier, astromech, demiurge and cosplay-studio, with a hand-classified sample of 72, all 65 fail and blocked receipts, and the agents' transcripts:
- No policy requires a passing receipt to close or merge (evidence.attached was removed from atelier's policy in 5c4a38ef); only the newer-failure block remains (crates/atelier-cli/src/commands/workflow.rs, close; crates/atelier-app/src/review_gate.rs, merge).
- Sample: 17 useful (24%), 14 moderate (19%), 41 busywork (57%). Busywork is mostly whole-suite, fmt and diff-check captures already gated by CI or merge checks; atelier-ksx6 got 18 receipts in 23 minutes.
- Base merges caused 47 re-records in atelier and 90 in astromech; 8 were required by an earlier fail. `merge --sync` reran 28 receipts in atelier, 26 never read. 49 merge refusals told agents to record fresh proof.
- Useful: browser checks (cosplay-studio-epm4), runtime checks on a build host (cosplay-studio-9mx9), real defects found in validation (astromech-t33g), before-fix fails (astromech-veqs), measurements (atelier-8ud1 latency), mission criteria receipts.
- Reviewers rerun checks themselves, as the review brief instructs, and rarely rely on receipts.
