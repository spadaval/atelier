---
schema: atelier.review
schema_version: 2
id: atelier-osjk
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: grok-plan-reviewer (grok-4.6)
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:27:39.940934057Z
event:
  event: finding
  recommendation: fix_now
---

Acts are specified as the installed atelier, which cannot write redesigned records until the cutover. docs/strategies/desk.md:99-103 and .atelier/issues/atelier-h3hl.md:27 say the Desk runs the installed atelier in a scratch worktree. Until wd1i, that binary is the old tool; this mission builds against converted clones (desk.md:148-149) and proves act-is-a-record in the Desk suite before go-live. Following the Design, those tests and h3hl fail or write the old layout. Fix: name the new binary (next/target/debug/atelier, then the post-cutover install) as the command the Desk runs, including for the act-is-a-record test.
