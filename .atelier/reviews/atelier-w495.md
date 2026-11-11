---
schema: atelier.review
schema_version: 2
id: atelier-w495
title: finding on atelier-8myy @5f7737c1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 5f7737c1d5c68bc04379a944f589437a75e86734
created_at: 2026-11-11T02:00:36.098824224Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-opus-5-5
    commit: 5a990a0a0936a584757c61d11d7ca37800f738ba
    recorded_at: 2026-11-11T02:04:16.772227807Z
    follow_up: atelier-m2v1
    reason: Deferred to backlog follow-up atelier-m2v1 under atelier-dd3a.
---

R5 medium: next/crates/atelier-cli/src/plan.rs:375 A started item whose branch is already broken cannot be dropped. Independently reproduced the Report's Surprise: with a valid base record and a branch committing status: broken, drop demo-ab12 --reason Cancelled on the base exits 2 parsing the branch before writing its terminal record. DEFER because this predates the patch and creating the drop is outside this item's terminal-precedence Done when line. Follow up by making drop use the base definition without needing a readable branch, with a regression. Review outcome at 5f7737c1: merge after R1-R4 are fixed, no approval now. Sizes: complexity medium (snapshot reads and holder-perspective conditions across app/CLI), value high (accurate approvals and terminal orientation), risk medium (misleading or unavailable workflow guidance). Contract fail; fitness fail on the supported cases above. Fresh checks passed: root and next fmt and locked metadata, exact merge whitespace check plus whole-range whitespace check, next clippy all-targets -D warnings, 227/227 nextest tests with four jobs/test threads, next-loop 10/10 and next-loop-changes 9/9. Read all changed code/tests/docs; Words, Lifecycle and Review are byte-for-byte unchanged. Did not rerun the destructive isolated CI script or the author's master-code mutation runs. No unrelated moving parts found.
