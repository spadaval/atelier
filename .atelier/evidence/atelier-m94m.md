---
created_at: "2026-06-19T04:18:08.273099835+00:00"
id: "atelier-m94m"
evidence_type: "validation"
captured_at: "2026-06-19T04:18:08.273098488+00:00"
target:
  kind: "issue"
  id: "atelier-kuuw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-kuuw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "ADR 0011 added native review modes and room authority: mutually exclusive review.mode room/provider, .atelier/reviews/<id>.yaml room records derived from ordered events, structured issue review field, hard atelier pr to atelier review rename, and review merge authority without issue workflow transitions. ADR 0010 now states provider-first pr/pull_request decisions are superseded by ADR 0011. Proof run: rg -n 'atelier pr|pull_request|review.mode|review merge' docs/adr CONTEXT.md reviewed; git diff --check -- '*.md' passed; atelier lint atelier-kuuw passed."
updated_at: "2026-06-19T04:18:11.158938911+00:00"
---

ADR 0011 added native review modes and room authority: mutually exclusive review.mode room/provider, .atelier/reviews/<id>.yaml room records derived from ordered events, structured issue review field, hard atelier pr to atelier review rename, and review merge authority without issue workflow transitions. ADR 0010 now states provider-first pr/pull_request decisions are superseded by ADR 0011. Proof run: rg -n 'atelier pr|pull_request|review.mode|review merge' docs/adr CONTEXT.md reviewed; git diff --check -- '*.md' passed; atelier lint atelier-kuuw passed.
