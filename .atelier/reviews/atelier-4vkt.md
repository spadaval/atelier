---
schema: atelier.review
schema_version: 2
id: atelier-4vkt
title: 'status.rs:70: in quiet mode text() computes policy diagnostics and drops them, and main.rs now skips its stderr print for Status/Prime, so ''atelier -q status'' on a broken policy shows nothing anywhere (master printed them to stderr). Print them to stderr in the quiet branch and assert it.'
target: atelier-sdb0
identity: Independent reviewer 2 (Claude subagent)
commit: 0f02bc674e5dc8c9a8f8422220cc748ff716783a
created_at: 2026-10-17T02:49:47.267877164Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: c8d931f95b2feb56681b580a4bd75349a9d9c238
    recorded_at: 2026-10-17T02:52:37.047878501Z
    resolution_commit: dba1c6045a6cb215255b127adc44c5c86e19581a
    reason: Quiet status/prime print policy diagnostics to stderr; test asserts it.
    migrated_from: atelier-o92z
---

status.rs:70: in quiet mode text() computes policy diagnostics and drops them, and main.rs now skips its stderr print for Status/Prime, so 'atelier -q status' on a broken policy shows nothing anywhere (master printed them to stderr). Print them to stderr in the quiet branch and assert it.
