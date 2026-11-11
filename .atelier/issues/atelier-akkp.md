---
created_at: "2026-11-06T22:08:25.310487900+00:00"
id: "atelier-akkp"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:49.658813670+00:00"
status: "declined"
title: "Re-measure record bookkeeping in adopting repositories after the lighter workflow ships"
updated_at: "2026-11-11T03:55:49.658813670+00:00"
---

## Goal

Check that the lighter workflow (mission atelier-n4mj) cut bookkeeping in real use, not only in the scenario test: measure the agent effort it was meant to cut on repositories using the released version: per merged item, the commands agents ran to record proof and review, the receipts recorded and re-recorded, and the refusals they had to clear; plus the findings-to-files count from the mission's Why section.

## Done when

A note on this issue gives those measures, from records, git history and agent transcripts, for a window of at least three weeks after the repositories adopted the release, next to the same measures for 2026-10-15 to 2026-11-06 (the evidence audit in atelier-s66u's Background is the method for receipts and refusals), and files a bug for any that did not fall. Record-only commit counts may be reported but are not a target.

## Context

Pick this up three or more weeks after the release containing atelier-n4mj's changes has been adopted in atelier, astromech, demiurge, cosplay-studio or life-on-rails. life-on-rails adopted Atelier on 2026-11-06 and follows the evidence rule voluntarily, so it is also a comparison point from before the release. It is outside the mission because it waits on calendar time and adoption. The original counts came from a temporary script; atelier-5fus's and atelier-rkxm's receipts record reusable methods.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It measures the evidence receipts and refusals of the old lighter workflow, and the What is removed table removes evidence records; the redesign's scorecard (atelier-c1z3, atelier-pwhn) measures the cost from here.
