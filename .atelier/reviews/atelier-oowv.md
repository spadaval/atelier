---
schema: atelier.review
schema_version: 2
id: atelier-oowv
title: finding on atelier-zpmb @c2642b25
target: atelier-zpmb
identity: gpt-6.1-sol-r1
commit: c2642b25f3d6053b00dfea9b623a50e9362dfb71
created_at: 2026-11-09T16:57:02.473468903Z
event:
  event: finding
  recommendation: fix_now
---

R3 medium: crates/atelier-site/src/lib.rs:157-178 holds the semaphore permit in the async request future rather than in the spawn_blocking job. When a client disconnects, the handler drops its permit but the blocking build keeps running, so new requests start more than the promised two builds. Reproduced through the real HTTP server with a valid fixture and a git wrapper that delays log calls: start two cold debrief requests, wait until both builds enter git log, close both sockets, then start two replacement requests. Four build log calls overlap and the replacement requests return 200. The ordinary build-cap test never cancels a request, so it misses this load-bound violation on the phone-serving host. Transfer an owned semaphore permit into the blocking closure so it remains held until the actual job ends, including after handler cancellation, and add a disconnect or aborted-future concurrency regression test.
