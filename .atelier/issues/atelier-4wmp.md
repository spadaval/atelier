---
created_at: "2026-06-23T15:22:33.355288019+00:00"
id: "atelier-4wmp"
issue_type: "feature"
labels:
- "cli"
- "human-output"
- "issue"
- "workflow-state"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Refresh issue queue, search, blocker, and objective-status output"
updated_at: "2026-06-23T20:24:32.904984858+00:00"
---

## Description

Refresh the highest-traffic work selection surfaces.

## Outcome

- `issue list`, `issue list --ready`, `search`, `issue blocked`, and
  `issue status <objective-id>` stop repeating per-row drill-down commands.
- Human summaries avoid telemetry-style `key=value` blobs and use readable
  counts.
- Parent/context labels are understandable, blocker rows prioritize human
  meaning, and objective status keeps health and next action visible.
- Work-selection output distinguishes executable, selectable, context-only,
  blocked, and blocked-through-parent rows so parent blockers and contextual
  visibility cannot be mistaken for ready work.
- `status`, `issue status <objective-id>`, and queue/search surfaces do not
  contradict Git or hide current workflow state; any remaining config/parser
  failure gives a public recovery path.
- `search` output and help use scope language that matches what is actually
  searched, such as issues versus broader records.

## Evidence

- Before/after transcripts cover blocked queues, ready queues, search results,
  blocker detail, and objective status with blocked work.
- Before/after transcripts or focused tests reproduce the actual complaint
  cases for hidden ready work, parent-blocker ambiguity, and status output that
  disagrees with Git or stale status-like output.
- Focused tests cover large blocker sets, long titles, omitted rows, and footer
  command deduplication, plus actionability labels for blocked-through-parent
  and context-only rows.
- `target/debug/atelier lint`, focused CLI tests, and `git diff --check` pass.
