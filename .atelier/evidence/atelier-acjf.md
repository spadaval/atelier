---
created_at: "2026-06-19T04:57:09.521056154+00:00"
id: "atelier-acjf"
evidence_type: "validation"
captured_at: "2026-06-19T04:57:09.521054950+00:00"
target:
  kind: "issue"
  id: "atelier-onkp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-onkp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Native room merge authority is implemented and validated. atelier review merge in room mode never changes Atelier workflow status; it requires at least one current approval and zero unresolved blocking findings, records a merged event, and sets the review room status to merged. Proof: cargo test -p atelier-app review_room --lib passed; cargo test -p atelier-cli pr --lib passed; disposable room-mode CLI smoke rejected unsafe merge states and then merged after resolving the blocking finding and adding a current approval."
updated_at: "2026-06-19T04:57:12.240558395+00:00"
---

Native room merge authority is implemented and validated. atelier review merge in room mode never changes Atelier workflow status; it requires at least one current approval and zero unresolved blocking findings, records a merged event, and sets the review room status to merged. Proof: cargo test -p atelier-app review_room --lib passed; cargo test -p atelier-cli pr --lib passed; disposable room-mode CLI smoke rejected unsafe merge states and then merged after resolving the blocking finding and adding a current approval.
