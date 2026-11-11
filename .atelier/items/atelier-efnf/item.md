---
schema: atelier.item
schema_version: 3
id: atelier-efnf
title: Show an open finding in branch readiness, so the approval mark agrees with merge
status: draft
mission: atelier-4pzj
labels:
- orientation
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-22T22:30:14.044062479Z
---

## Goal

The status Branches row's `✓ approved` reflects the approval alone (`review_gate::approval_state`, read by `Readiness::facts` in crates/atelier-cli/src/commands/git.rs), so a branch reads approved while `merge` refuses it for an open finding. Under atelier-md0v a `fix_now` finding is resolved only by an approval recorded after it, and a `defer` or `no_action` finding still needs a decision (`findings.disposed`), so a finding recorded after the approval, or a deferral nobody decided, leaves the row saying ✓ approved. Found in review of atelier-tciw (finding atelier-77lm).

## Done when

- [ ] An owner with a current approval and an open finding (a `fix_now` finding recorded after the approval, or a `defer` finding with no decision) shows it in its readiness facts, e.g. `✓ approved, ⚠ 1 open finding`, judged by the rule `findings.disposed` uses. Tests in crates/atelier-cli/tests/session.rs cover both kinds, and a resolved finding that shows nothing.
- [ ] docs/product/human-cli-output.md (Branches) says what the approval mark and the finding fact cover.

## Context

Decision atelier-mzpn meant this agreement for atelier-md0v, which merged without it. Dropped there: assertions for a self-approved row, a trivial exemption, readiness unknown, superseded children exempt from proof and the Shared files cap (finding atelier-eqyw), which tested rules mmpv removed. The readiness facts still print `⚠ failing evidence <id>`, and the code still calls it a merge blocker, though receipts gate nothing since md0v; say in the Report whether that fact stays. atelier-abdp adds a review-round count to the same facts.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: atelier-77lm
