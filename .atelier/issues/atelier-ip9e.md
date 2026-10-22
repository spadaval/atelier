---
created_at: "2026-10-15T19:30:40.395944986+00:00"
id: "atelier-ip9e"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-ip9e
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "atelier-byp9"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:19:53.789845012+00:00"
status: "done"
title: "Avoid cascade missing-target errors after a duplicate record ID"
updated_at: "2026-10-16T19:19:53.789845012+00:00"
---

## Goal

`atelier check` reports a cross-kind duplicate record ID once, without also reporting every reference to the dropped record as missing.

## Done when

A fixture with evidence sharing an issue ID and another record relating to it reports only the duplicate-ID error; fail-fast loading still returns the same first error.

## Context

Review finding atelier-byp9 on one-pass graph validation (fdf8fd68): crates/atelier-app/src/record_index.rs structural() drops duplicate-kind records, so references to them also report as missing. Low impact: random IDs make collisions unlikely, and the first error is unchanged.

## Close Reason

RecordIndex::structural remembers the IDs it drops as duplicates and removes 'reference to missing <kind> <id>' errors for those IDs, so check reports a cross-kind duplicate once; fail-fast loading still returns the duplicate error first. Test: duplicate_id_is_reported_once_without_missing_reference_cascade (evidence sharing an issue ID, referenced by another evidence record); with the filter disabled it also reports the missing evidence.
