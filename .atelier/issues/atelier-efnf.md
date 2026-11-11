---
created_at: "2026-10-22T22:30:14.044062479+00:00"
id: "atelier-efnf"
issue_type: "task"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "review"
    id: "atelier-77lm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:37.590147328+00:00"
status: "superseded"
title: "Show an open finding in branch readiness, so the approval mark agrees with merge"
updated_at: "2026-11-11T03:55:37.590147328+00:00"
---

## Goal

The status Branches row's `✓ approved` reflects the approval alone (`review_gate::approval_state`, read by `Readiness::facts` in crates/atelier-cli/src/commands/git.rs), so a branch reads approved while `merge` refuses it for an open finding. Under atelier-md0v a `fix_now` finding is resolved only by an approval recorded after it, and a `defer` or `no_action` finding still needs a decision (`findings.disposed`), so a finding recorded after the approval, or a deferral nobody decided, leaves the row saying ✓ approved. Found in review of atelier-tciw (finding atelier-77lm).

## Done when

- [ ] An owner with a current approval and an open finding (a `fix_now` finding recorded after the approval, or a `defer` finding with no decision) shows it in its readiness facts, e.g. `✓ approved, ⚠ 1 open finding`, judged by the rule `findings.disposed` uses. Tests in crates/atelier-cli/tests/session.rs cover both kinds, and a resolved finding that shows nothing.
- [ ] docs/product/human-cli-output.md (Branches) says what the approval mark and the finding fact cover.

## Context

Decision atelier-mzpn meant this agreement for atelier-md0v, which merged without it. Dropped there: assertions for a self-approved row, a trivial exemption, readiness unknown, superseded children exempt from proof and the Shared files cap (finding atelier-eqyw), which tested rules mmpv removed. The readiness facts still print `⚠ failing evidence <id>`, and the code still calls it a merge blocker, though receipts gate nothing since md0v; say in the Report whether that fact stays. atelier-abdp adds a review-round count to the same facts.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new `status` lists only what is not met on an item, open fix-now findings included, and `verdict approve` is refused while a fix-now finding is open (product specification, What status prints and Reviewer).
