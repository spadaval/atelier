---
created_at: "2026-10-16T23:43:33.771473753+00:00"
id: "atelier-kops"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-kops
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
  - kind: "review"
    id: "atelier-0c86"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T06:05:33.756782586+00:00"
status: "done"
title: "Owned branches trust a local base that may hold unchecked commits"
updated_at: "2026-10-17T06:05:33.756782586+00:00"
---

## Goal

land on an owned branch trusts history up to its branch base, the local base tip it started from. The local base can hold commits no land has checked, so a branch started from an unchecked local commit gets a passing land. The base's own land against its upstream catches this before a push, so nothing unchecked reaches the upstream; this closes the gap earlier.

## Done when

On an owned branch, land trusts history only up to the merge base of the branch base with the base's upstream (all claims checked without an upstream), with a test that starts work from an unpushed local base commit carrying failing proof.

## Context

Found in review of atelier-37uq (finding atelier-0c86), which made unowned branches trust only the upstream.

## Close Reason

On an owned branch, land trusts history only up to the merge base of the branch base with the base's upstream, and checks every claim without an upstream (session.rs land_text). The extended land_checks_claims_that_linked_records_or_removed_proof_affect test starts owned work from an unpushed local base commit with failing proof and shows land refuses it with and without an upstream; it fails with the fix reverted. Docs: sessions-and-hooks.md and ADR 0019. Full suite with test hooks passes at 9420c2c7.
