---
created_at: "2026-10-15T21:00:45.352896436+00:00"
id: "atelier-4n7v"
issue_type: "feature"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let bundles revise existing records, and report client_ref to ID"
updated_at: "2026-10-15T21:00:45.352896436+00:00"
---

## Goal

Bundles are create-only, and `issue update --body` only works on missions. Revising a large imported graph after review meant deleting every record and applying again, which gave every record a new ID. That's tolerable before the first merge, but not once IDs are referenced. `bundle apply` also prints relationships by allocated ID only, with no client_ref→ID mapping or titles, so it's hard to check the result.

## Done when

A bundle can update bodies, fields and links of existing records by ID with the same validation as create, and apply prints (or writes) the client_ref→ID mapping with titles; tests cover an update and the mapping.

## Context

astromech: three regenerations of a 153-record bundle during review.

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.
