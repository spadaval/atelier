---
schema: atelier.review
schema_version: 1
id: atelier-4vkt
title: 'status.rs:70: in quiet mode text() computes policy diagnostics and drops them, and main.rs now skips its stderr print for Status/Prime, so ''atelier -q status'' on a broken policy shows nothing anywhere (master printed them to stderr). Print them to stderr in the quiet branch and assert it.'
target: atelier-sdb0
identity: Independent reviewer 2 (Claude subagent)
commit: 0f02bc674e5dc8c9a8f8422220cc748ff716783a
created_at: 2026-10-17T02:49:47.267877164Z
event:
  event: finding
  recommendation: fix_now
---

status.rs:70: in quiet mode text() computes policy diagnostics and drops them, and main.rs now skips its stderr print for Status/Prime, so 'atelier -q status' on a broken policy shows nothing anywhere (master printed them to stderr). Print them to stderr in the quiet branch and assert it.
