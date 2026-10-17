---
schema: atelier.review
schema_version: 1
id: atelier-v3jp
title: 'crates/atelier-app/src/mission_review.rs:229: review events are now read only when they target scoped items, so a finding on the mission itself (for example a human''s plan review) that is deferred to a follow-up outside the scope never reaches Open work. The old code added the mission to the reviewed set and listed those follow-ups. Reproduced: review finding <mission> plus disposition defer --follow-up U. U is absent from the debrief. Fix: include the mission''s own deferred findings in follow_up_source, with the mission as the source.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:48.880233392Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/mission_review.rs:229: review events are now read only when they target scoped items, so a finding on the mission itself (for example a human's plan review) that is deferred to a follow-up outside the scope never reaches Open work. The old code added the mission to the reviewed set and listed those follow-ups. Reproduced: review finding <mission> plus disposition defer --follow-up U. U is absent from the debrief. Fix: include the mission's own deferred findings in follow_up_source, with the mission as the source.
