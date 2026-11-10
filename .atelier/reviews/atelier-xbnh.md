---
schema: atelier.review
schema_version: 2
id: atelier-xbnh
title: finding on atelier-fcb7 @22c23c70
target: atelier-fcb7
identity: gpt-6-astra-amend-r1
commit: 22c23c7036925cfc15ad9d9510c0f6f32480e41e
created_at: 2026-11-10T02:43:27.757289763Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:386: The runbook installs pre-push before pushing conversion commits that change config, AGENTS.md and skill files, then explicitly requires plain git push. Its new base rule necessarily refuses these pushes. Rollback also leaves the new shared Git hooks installed: restoring records can invoke the new validator on old data, and the fallback setup only replaces Claude settings, so remaining hooks invoke unsupported check --hook modes after fallback installation. Smallest fix: specify ATELIER_SKIP with the reviewed conversion reason for conversion pushes, save and restore the prior Git hook installation before reverting records or restoring the old binary, and exercise forward and rollback paths with real hooks enabled in the existing rehearsal.
