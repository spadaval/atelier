---
schema: atelier.review
schema_version: 2
id: atelier-e93x
title: finding on atelier-wd1i @d41d3bcd
target: atelier-wd1i
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:11.926023058Z
event:
  event: finding
  recommendation: fix_now
---

high: .atelier/issues/atelier-wd1i.md:48 G claims success by checking all real checkouts after F deliberately leaves a product checkout on its old feature branch while updating only its base. next/crates/atelier-cli/src/repo.rs:36 reads config from that checkout, which has no new config. check.rs:30 enumerates new-format records and does not require config, so a check pass alone can even conceal an unusable checkout with zero new records. Require the runbook to leave every resumed checkout on converted records or explicitly quarantined with a reviewed recovery path, and verify status and expected record counts in the actual checkouts, not only fresh conversion clones. This must be settled with F before the day.
