---
created_at: "2026-06-10T00:34:04.562255176+00:00"
id: "atelier-0025"
issue_type: "task"
labels:
- "assignee:root"
- "export"
- "manifest"
- "rebuild"
- "storage"
- "task"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0023"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Remove manifest.json from canonical state"
updated_at: "2026-06-10T01:14:14.098013254+00:00"
---

## Description

Eliminate manifest.json as a canonical source-of-truth file. Rebuild should discover Markdown record files directly and validate path/id/schema/front matter without a generated inventory. Export/check should compare rendered records against the tree and report stale, missing, extra, malformed, duplicate, and dangling records without relying on manifest hashes.

## Outcome

manifest.json is no longer required in .atelier-state; rebuild/export/check/lint work from discovered record files; tests cover missing manifest, extra files, duplicate IDs, path/id mismatch, schema mismatch, malformed front matter, and stale rendered output; docs and fixtures no longer describe manifest.json as canonical.

## Evidence

Evidence was not specified in the legacy issue record.
