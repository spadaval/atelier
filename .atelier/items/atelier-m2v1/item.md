---
schema: atelier.item
schema_version: 3
id: atelier-m2v1
title: Let `drop` end a started item whose branch record does not parse
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T02:04:16.772227807Z
---

## Goal

Deferred from finding atelier-w495 on atelier-8myy @5f7737c1, by `atelier review defer`.

R5 medium: crates/atelier-cli/src/plan.rs:375 A started item whose branch is already broken cannot be dropped. Independently reproduced the Report's Surprise: with a valid base record and a branch committing status: broken, drop demo-ab12 --reason Cancelled on the base exits 2 parsing the branch before writing its terminal record. DEFER because this predates the patch and creating the drop is outside this item's terminal-precedence Done when line. Follow up by making drop use the base definition without needing a readable branch, with a regression. Review outcome at 5f7737c1: merge after R1-R4 are fixed, no approval now. Sizes: complexity medium (snapshot reads and holder-perspective conditions across app/CLI), value high (accurate approvals and terminal orientation), risk medium (misleading or unavailable workflow guidance). Contract fail; fitness fail on the supported cases above. Fresh checks passed: root and next fmt and locked metadata, exact merge whitespace check plus whole-range whitespace check, next clippy all-targets -D warnings, 227/227 nextest tests with four jobs/test threads, next-loop 10/10 and next-loop-changes 9/9. Read all changed code/tests/docs; Words, Lifecycle and Review are byte-for-byte unchanged. Did not rerun the destructive isolated CI script or the author's master-code mutation runs. No unrelated moving parts found.

## Done when

- [ ] `atelier drop` on the base ends a started item whose branch record does not parse, using the base's record, and a test covers it.

## Links

Relationships of the old record that the new format has no field for.

- parent: atelier-dd3a (Deferred review findings on atelier-8myy)
- discovered_from review: atelier-w495
