---
created_at: "2026-06-23T16:21:20.072142428+00:00"
id: "atelier-95rr"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Specify selected-domain-fact cache schema"
updated_at: "2026-06-23T16:21:20.072142428+00:00"
---

## Description

Design the target SQLite cache schema around selected domain facts needed by command behavior, including issues, evidence, reviews, links, labels, and cache source metadata. The cache is not a generic graph and is not a partial object mirror; each cached field or relation should be justified by common filtering, sorting, graph traversal, validation, or lookup needs.

## Outcome

- The target schema removes the current issue-plus-generic-record hybrid as the desired architecture.
- The design states which full domain data remains in record files rather than cache rows.
- The design keeps SQLite as disposable cache and treats schema/cache-version mismatch as rebuildable runtime state, not durable migration state.
- The design names the per-record-type indexing paths that full rebuild and incremental changed-record repair should share.

## Evidence

- File changes in architecture docs or ADRs specify the selected-domain-fact SQLite cache schema.
- File changes in architecture docs identify which full domain data remains in record files rather than cache rows.
- `atelier lint atelier-95rr` passes after the schema design update.
