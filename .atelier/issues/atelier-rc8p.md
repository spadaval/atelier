---
created_at: "2026-10-17T19:00:36.353441447+00:00"
id: "atelier-rc8p"
issue_type: "feature"
labels:
- "retro-2026-10-17"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add atelier review request to generate the reviewer brief"
updated_at: "2026-10-17T19:00:36.353441447+00:00"
---

## Goal

Orchestrators hand-wrote the reviewer prompt about 113 times in the adoption run (about 12% of its output tokens): identity, TMPDIR, target dir, scope commit, Done when, prior findings.

## Done when

`atelier review request <id>` prints a dense reviewer brief: the item's Goal and Done when, the commit range to review, prior open findings and dispositions, the commands for recording findings and approval, and the reviewer's working rules from policy. A rendering test pins it; the Agent Factory skill uses it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
