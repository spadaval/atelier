---
created_at: "2026-11-05T04:40:10.371706005+00:00"
id: "atelier-okfy"
issue_type: "bug"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:56.059858532+00:00"
status: "superseded"
title: "Show the mission's own Report and item notes in the debrief"
updated_at: "2026-11-11T03:55:56.059858532+00:00"
---

## Goal

The debrief does not show the mission's own Report, the report written when it closes (outcome, deviations, decisions, surprises, open items; read from `## Report`, or `## Close Reason` in older records), including vt8q's explanation that two later code commits don't affect S1, S3 or S4. Item notes such as g8cn's v0.3.1 publication note are missing too, although Left open points at them. Under atelier-jnrh each item's Report is written before review and carried into the merge, and a mission closes once all its work is merged and its validation, if it has one, is done.

## Done when

- [ ] A CLI fixture in a scratch repository shows the mission's Report and the item notes Left open points at, in both the Markdown debrief and the site's debrief page, for a mission closed under the current lifecycle and for one whose records use `## Close Reason`.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8. vt8q's records become prunable after 2026-11-11, so a fixture is the check. Its mission, atelier-gg6b, was superseded in decision atelier-mzpn; this item stands alone until loose end atelier-jdpf reviews a debrief generated under the new lifecycle.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes the debrief, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief.
