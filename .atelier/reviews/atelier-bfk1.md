---
schema: atelier.review
schema_version: 2
id: atelier-bfk1
title: finding on atelier-uwxy @125a72b3
target: atelier-uwxy
identity: Independent planning reviewer (GPT-6 Astra)
commit: 125a72b312eeea6a249ca4d5b93240d39e61d357
created_at: 2026-11-11T05:30:09.772295672Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-uwxy.md:62: The revised instruction explicitly runs atelier merge --sync after the cutover, but that cutover replaces the installed CLI. next/crates/atelier-cli/src/main.rs:198-202 defines Merge with only --trivial; docs/strategies/redesign-architecture.md:1053-1056 says merge_sync is not carried over. A worker following this recovery step gets an unsupported option. Replace it with bringing the checkout forward under the reviewed cutover runbook and the new workflow (the new CLI's conflict guidance at next/crates/atelier-cli/src/output.rs:1206 uses git merge master), and reserve atelier merge --sync for before cutover if needed. Recommendation: FIX NOW; the changed instruction names a command unavailable in its stated environment.
