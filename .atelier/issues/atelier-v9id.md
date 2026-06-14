---
created_at: "2026-06-12T06:05:00+00:00"
id: "atelier-v9id"
issue_type: "validation"
labels:
- "adversarial-validation"
- "mission"
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-zue4"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T01:40:28.846769072+00:00"
status: "done"
title: "Adversarially validate repair mission outcomes"
updated_at: "2026-06-13T01:40:28.846769072+00:00"
---

## Description

Run a mission-wide adversarial validation pass after implementation work lands.
This validation starts from the mission and epic Outcome sections, not from
implementation summaries. The validator should try to prove that the claimed
repairs are still missing, misleading, or only partially wired.
- Every mission validation criterion is mapped to concrete linked work,
  command transcripts, tests, docs, Agent Factory guidance, or attached evidence.
- Every linked epic Outcome line is classified as pass, fail, blocked,
  deferred, or not-applicable with a reason.
- Positive behavior and negative behavior are both validated: new surfaces work,
  obsolete surfaces are absent, hidden, or clearly non-normal, and failure gates
  fail when required proof is missing.
- The validator checks docs, help output, Agent Factory skill text, stale tests,
  ignored tests, projection freshness behavior, and tracker lint/doctor/export
  health instead of only running broad tests.
- The validator checks that the validation process itself caught seeded
  missing-proof and independence failures before relying on mission closeout
  evidence.
- Findings create or identify follow-up tracker items before mission closeout.
- The validation pass is performed by an agent that did not implement the
  slices being validated.
- Attach a mission contract-audit evidence record that maps mission and epic
  Outcome lines to proof.

- Capture representative positive and negative command transcripts for issue
  sections, lint/start/closeout gates, command-surface cleanup, status/start/work
  surfaces, workflow-validator removal, evidence capture, projection freshness,
  and reliability status.

- Record stale or ignored test inventory results and the linked owner for each
  unresolved item.

- Record docs/help/Agent Factory guidance parity results in the mission
  contract-audit evidence record.

- Record validation-system failure scenario results in an attached evidence
  record.

- Run focused integration tests for the repaired surfaces.

- Run `cargo fmt -- --check`, `git diff --check`, `atelier export --check`,
  `atelier lint`, and `atelier doctor`.
This is not a code review. It is a behavioral closeout challenge. The validator
should not fix defects while validating; failures become blocker findings or
new repair work.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
