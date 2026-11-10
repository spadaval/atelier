---
schema: atelier.review
schema_version: 2
id: atelier-6tiq
title: finding on atelier-fcb7 @291c704b
target: atelier-fcb7
identity: gpt-6-astra-amend3-r1
commit: 291c704bf6337152af49f101a1aaecd5dcca33e8
created_at: 2026-11-10T04:47:43.690647467Z
event:
  event: finding
  recommendation: fix_now
---

medium atelier-mal9: byte-for-byte Markdown-body preservation conflicts with copying front-matter splitting with its behaviour unchanged. lib.rs:1332-1350 strips an optional leading and trailing newline, making different input bodies indistinguishable; a writer cannot recover both. Explicitly allow changing that boundary to retain the raw body slice and test absent versus present boundary newlines. Also point ID generation at src/record_id.rs, which lib.rs only calls, so the named copy source includes the actual allocator.
