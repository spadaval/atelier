---
schema: atelier.review
schema_version: 2
id: atelier-0ig8
title: 'crates/atelier-app/src/lint.rs:147: criteria findings run only when every other finding is empty, so any unrelated issue finding hides criteria findings and they appear only after the first batch is fixed. The gate is needed for RecordIndex::load on invalid records, but issue_findings/strategy_findings do not make the index unloadable. Fix: gate on the integrity findings only.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:23.712011038Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e
    recorded_at: 2026-10-17T18:41:32.732299386Z
    resolution_commit: fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e
    reason: Fixed in fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e with tests; see the commit message
    migrated_from: atelier-lwjv
---

crates/atelier-app/src/lint.rs:147: criteria findings run only when every other finding is empty, so any unrelated issue finding hides criteria findings and they appear only after the first batch is fixed. The gate is needed for RecordIndex::load on invalid records, but issue_findings/strategy_findings do not make the index unloadable. Fix: gate on the integrity findings only.
