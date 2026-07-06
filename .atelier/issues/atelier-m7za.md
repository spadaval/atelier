---
created_at: "2026-06-23T16:21:20.083529820+00:00"
id: "atelier-m7za"
issue_type: "validation"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Validate lazy cache behavior for batch writes"
updated_at: "2026-07-06T20:57:19.010435831+00:00"
---

## Description

Add regression coverage showing relationship-heavy batch writes do not rebuild the full cache after each mutation and the next cache query returns correct graph results.

## Outcome

- Tests or scripted evidence prove lazy cache behavior for the original slow batch scenario.

## Evidence

- Independent scenario evidence `atelier-x1d1` and transcript
  `/tmp/atelier-mska-m7za-transcript.txt` prove that a relationship-heavy
  `bundle apply` left `state.db` byte-for-byte unchanged, stale state was
  detectable, and the next cache-backed query performed exactly one repair and
  returned the correct mission, parent, blocker, and evidence relationships.
- Test evidence `atelier-vj0b` records 67/67 focused cache tests passing. The
  448-test CLI inventory contains no ignored tests, so no ignored or skipped
  test is used as proof.
