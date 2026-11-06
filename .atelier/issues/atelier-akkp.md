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
status: "backlog"
title: "Re-measure record bookkeeping in adopting repositories after the lighter workflow ships"
updated_at: "2026-11-06T22:08:25.310487900+00:00"
---

## Goal

Check that the lighter workflow (mission atelier-n4mj) cut bookkeeping in real use, not only in the scenario test: rerun the record-only commit, findings-to-files and repeat-approval counts from the mission's Why section on repositories using the released version.

## Done when

A note on this issue gives the same counts as docs/strategies/lighter-workflow.md's Why section, for a window of at least three weeks after the repositories adopted the release, next to the original figures, and files a bug for any count that did not fall.

## Context

Pick this up three or more weeks after the release containing atelier-n4mj's changes has been adopted in atelier, astromech, demiurge, cosplay-studio or life-on-rails. It is outside the mission because it waits on calendar time and adoption. The original counts came from a temporary script; atelier-5fus's and atelier-rkxm's receipts record reusable methods. The raw record-only share includes planning commits, which the mission does not target, so also count the record-only commits per merged work branch, which is what the mission's lifecycle-cost scenario measures.
