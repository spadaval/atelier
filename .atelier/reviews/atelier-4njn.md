---
schema: atelier.review
schema_version: 2
id: atelier-4njn
title: finding on atelier-zrss @5959a1ba
target: atelier-zrss
identity: reviewer-sol-r1
commit: 5959a1ba4d5f77746047b7eca95f9699bc275362
created_at: 2026-11-11T00:16:25.598204296Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/output.rs:1918: R2. A merge-check command is inserted verbatim into a skipped rule, although a valid TOML run string can contain newlines. Reproduced with run containing echo broken followed by a newline and false: merge with ATELIER_SKIP exits 0, but the Atelier-Skip value contains an unindented second line. git log -1 --format=%(trailers) main then returns no trailers at all, losing recognition of Atelier-Merge as well as Atelier-Skip; the pre-push hook uses that parsed Atelier-Merge trailer to recognize a legitimate merge. This breaks the required commit history and later publishing. Normalize each rule to one line before recording or formatting it, and test a skipped multiline merge check using Git to parse both trailers.
