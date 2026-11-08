---
created_at: "2026-11-08T05:35:32.496582982+00:00"
id: "atelier-jy8k"
issue_type: "loose_end"
labels:
- "agent-factory"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-l58v"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Schedule the gardener to run daily on Sumeet's Atelier repositories"
updated_at: "2026-11-08T05:35:32.496582982+00:00"
---

## Goal

Run the gardener procedure daily through a T3 Code scheduled task once the skill change merges.

## Done when

- [ ] A T3 scheduled task runs the gardener daily on atelier, astromech and life-on-rails (others on Sumeet's say), with heavy builds sent to boat, and its first run's output is checked by hand.

## Context

Waits on the skill item that adds procedures/garden.md.
