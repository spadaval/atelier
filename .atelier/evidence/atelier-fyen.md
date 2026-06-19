---
created_at: "2026-06-19T04:56:58.619463592+00:00"
id: "atelier-fyen"
evidence_type: "validation"
captured_at: "2026-06-19T04:56:58.619461959+00:00"
target:
  kind: "issue"
  id: "atelier-8uys"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-8uys"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Native room findings, decisions, and stale approval handling are implemented and validated. Room comments can create blocking or nonblocking findings; resolve records explicit finding resolution; request-changes invalidates earlier approvals so merge requires a later current approval. Proof: cargo test -p atelier-app review_room --lib passed; disposable room-mode CLI smoke showed merge rejected with review_room_blocking_findings, then rejected with review_room_not_approved after request-changes until a later approval."
updated_at: "2026-06-19T04:57:01.446079726+00:00"
---

Native room findings, decisions, and stale approval handling are implemented and validated. Room comments can create blocking or nonblocking findings; resolve records explicit finding resolution; request-changes invalidates earlier approvals so merge requires a later current approval. Proof: cargo test -p atelier-app review_room --lib passed; disposable room-mode CLI smoke showed merge rejected with review_room_blocking_findings, then rejected with review_room_not_approved after request-changes until a later approval.
