---
created_at: "2026-10-30T06:40:25.029767683+00:00"
id: "atelier-r9hb"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show a Done when edited after a finding or failing receipt"
updated_at: "2026-10-30T06:40:25.029767683+00:00"
---

## Goal

When an item's Done when (or a mission's success criteria) changes in a commit later than a finding or a failing receipt on that item, show it where closing is judged, so a criterion loosened to let an item close is seen by the reviewer and the human rather than hidden by a fresh approval.

## Done when

Tests show: for an item with a finding pinned at commit A, its Done when edited at a later commit B and an approval at C, `merge <owner> --check` still passes but prints one line naming the item, commit B and the finding, and `atelier debrief <mission>` lists the same edit under the item. The same flow with a failing receipt instead of a finding shows the same line. An item whose Done when changed only before its first finding or failing receipt shows no line. Reverting the detection makes these tests fail.

## Scope

`merge --check` and `merge` output, the debrief model (crates/atelier-app/src/mission_review.rs) and its renderers, docs/product/reviews-and-merge.md. The verdict and exit status do not change.

## Context

Record history from Git: crates/atelier-app/src/record_history.rs (`issue_histories`). Finding and receipt times come from their records' pinned commits.

## Design

Surface, do not refuse. A Done when is sometimes rightly corrected after a finding, and refusing would need an override or a human-only record, which ADR 0018 keeps Atelier from becoming. The line gives the reviewer and the debrief reader the fact to judge.
