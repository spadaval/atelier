---
schema: atelier.review
schema_version: 1
id: atelier-38nk
title: finding on atelier-lnwf @35aab9d1
target: atelier-lnwf
identity: Independent reviewer (Claude subagent)
commit: 35aab9d13627f1c7d86bf0a94199c010de586ddf
created_at: 2026-10-28T01:22:01.709651769Z
event:
  event: finding
  recommendation: defer
---

low: .github/workflows/ci.yml:16-19 — the workflow-wide RUST_BACKTRACE: 0 masks a pre-existing defect: main returns anyhow::Result, so with RUST_BACKTRACE set user-facing errors get a stack backtrace appended (run 40609860117, record_refs.rs:383). Developers with RUST_BACKTRACE=1 get 7 local test failures. Not introduced or worsened by this change; follow up by printing errors without backtraces, then drop the env.
