---
created_at: "2026-10-15T21:00:45.352896436+00:00"
id: "atelier-4n7v"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-4n7v
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T00:37:13.322777136+00:00"
status: "done"
title: "Let bundles revise existing records, and report client_ref to ID"
updated_at: "2026-10-16T00:37:13.322777136+00:00"
---

## Goal

Bundles are create-only, and `issue update --body` only works on missions. Revising a large imported graph after review meant deleting every record and applying again, which gave every record a new ID. That's tolerable before the first merge, but not once IDs are referenced. `bundle apply` also prints relationships by allocated ID only, with no client_ref→ID mapping or titles, so it's hard to check the result.

## Done when

A bundle can update bodies, fields and links of existing records by ID with the same validation as create, and apply prints (or writes) the client_ref→ID mapping with titles; tests cover an update and the mapping.

## Context

astromech: three regenerations of a 153-record bundle during review.

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.

## Close Reason

Bundle issue entries with operation update and id revise an existing record (title, body, priority, labels, strategy replace; links and notes add; status and type changes refused), with the same validation and staged-graph checks as create. Preview and apply list every client_ref with its ID and title. Documented in docs/spec/bundle/schema.md (Updating records). Link removal stays with atelier issue unlink. Test: test_bundles_update_existing_records_and_report_client_ref_ids.
