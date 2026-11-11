---
schema: atelier.review
schema_version: 2
id: atelier-310d
title: finding on atelier-efsl @57231b68
target: atelier-efsl
identity: gpt-6.1-sol
commit: 57231b68eb61df98c9748c0864b5b42599bc5154
created_at: 2026-11-11T00:59:07.095691610Z
event:
  event: finding
  recommendation: fix_now
---

Low: .atelier/issues/atelier-efsl.md:36 says status prints no start command whenever no ready item can start. On the base a thread may already hold a building or approved item with zero startable ready items: output.rs:534-536 deliberately says Then: atelier start <held> (switches to its branch). The new unconditional acceptance line would remove this useful existing guidance, beyond filtering the Ready to start section. Qualify the no-start requirement to callers holding no item, preserve the held-item return command, and verify this base-view case with a focused assertion.
