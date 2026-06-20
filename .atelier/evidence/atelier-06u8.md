---
created_at: "2026-06-19T04:56:47.761706509+00:00"
id: "atelier-06u8"
evidence_type: "validation"
captured_at: "2026-06-19T04:56:47.761705426+00:00"
target:
  kind: "issue"
  id: "atelier-at7i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-at7i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Native room open/status/show/comments/comment behavior is implemented and validated. Room mode writes .atelier/reviews/<id>.yaml, stores issue review.kind = room with the room id, appends opened/comment/finding events, refreshes projection, and renders review status/show/comments through atelier review. Proof: cargo test -p atelier-app review_room --lib passed; cargo test -p atelier-cli pr --lib passed; disposable room-mode CLI smoke opened a room, recorded a finding/comment event, and rendered review status."
updated_at: "2026-06-19T04:56:50.561931674+00:00"
---

Native room open/status/show/comments/comment behavior is implemented and validated. Room mode writes .atelier/reviews/<id>.yaml, stores issue review.kind = room with the room id, appends opened/comment/finding events, refreshes projection, and renders review status/show/comments through atelier review. Proof: cargo test -p atelier-app review_room --lib passed; cargo test -p atelier-cli pr --lib passed; disposable room-mode CLI smoke opened a room, recorded a finding/comment event, and rendered review status.
