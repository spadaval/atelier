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
title: "Show a Done when edited after the item was started"
updated_at: "2026-10-30T06:40:25.029767683+00:00"
---

## Goal

When an item's Done when is edited after the item was started, show the edit where closing is judged, so a criterion loosened to let an item close is seen by the reviewer and the human rather than hidden by a fresh approval. In the usage review most narrowings had no recorded finding before them, so the trigger is the start, not a finding.

## Done when

Tests show: for an item started at commit S, with its Done when edited at commit B that descends from S (Git ancestry, not timestamps) and an approval after B, `merge <owner> --check` still passes, with an unchanged exit status, but prints one line naming the item and commit B, and also the finding or failing receipt on the item pinned to an ancestor of B, when there is one. `atelier debrief <mission>` lists the same edit under the item. An item whose Done when changed only before S shows no line. Reverting the detection makes these tests fail.

## Scope

`merge --check` and `merge` output, the debrief model (crates/atelier-app/src/mission_review.rs) and its renderers, docs/product/reviews-and-merge.md. The verdict and exit status do not change. Mission success criteria are out of scope: their failing receipts sit on validation items and findings are rarely recorded on missions.

## Context

Record history from Git: crates/atelier-app/src/record_history.rs (`issue_histories`). Order edits, findings and receipts by Git ancestry of their commits; the debrief already mixes clocks (atelier-b3qt).

## Design

Surface, do not refuse. A Done when is sometimes rightly corrected after a finding, and refusing would need an override or a human-only record, which ADR 0018 keeps Atelier from becoming. The line gives the reviewer and the debrief reader the fact to judge.
