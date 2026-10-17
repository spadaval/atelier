---
created_at: "2026-10-17T19:00:36.356707263+00:00"
id: "atelier-kglz"
issue_type: "feature"
labels:
- "orientation"
- "retro-2026-10-17"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show docs that exist only on unmerged branches"
updated_at: "2026-10-17T19:00:36.356707263+00:00"
---

## Goal

'Where is ADR 22? I don't see it in the main tree' — it was on a review branch (872b89d8). Plans under review are invisible from the base checkout.

## Done when

`atelier status` and `work mission` list ADRs, strategies and product docs added on local owner branches but absent from the base, with the branch and its review state. A test covers it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).
