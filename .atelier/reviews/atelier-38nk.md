---
schema: atelier.review
schema_version: 2
id: atelier-38nk
title: finding on atelier-lnwf @35aab9d1
target: atelier-lnwf
identity: Independent reviewer (Claude subagent)
commit: 35aab9d13627f1c7d86bf0a94199c010de586ddf
created_at: 2026-10-28T01:22:01.709651769Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Independent reviewer (Claude subagent)
    commit: ad66d1e49a0a27e843b86fbf058b6badc4ad053f
    recorded_at: 2026-10-28T01:22:02.051100332Z
    follow_up: atelier-eb71
    reason: 'Reviewer''s recommendation: pre-existing CLI behavior outside this issue''s scope; the CI env is the minimal correct workaround. Tracked in atelier-eb71.'
    migrated_from: atelier-i7k6
---

low: .github/workflows/ci.yml:16-19 — the workflow-wide RUST_BACKTRACE: 0 masks a pre-existing defect: main returns anyhow::Result, so with RUST_BACKTRACE set user-facing errors get a stack backtrace appended (run 40609860117, record_refs.rs:383). Developers with RUST_BACKTRACE=1 get 7 local test failures. Not introduced or worsened by this change; follow up by printing errors without backtraces, then drop the env.
