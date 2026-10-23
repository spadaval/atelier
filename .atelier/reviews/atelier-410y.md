---
schema: atelier.review
schema_version: 1
id: atelier-410y
title: finding on atelier-0uaj @93e82219
target: atelier-0uaj
identity: Independent reviewer 5 (Claude Sonnet subagent)
commit: 93e822196a3baede9b848a94d926edbdd224e6be
created_at: 2026-10-23T17:02:22.048392943Z
event:
  event: finding
  recommendation: no_action
---

info: append_only bails on the first offending commit, so two bad commits need two land runs; the two printed rebases compose (verified bad1/bad2 with later work between: each fix then rerun land, ends on review.current/findings.disposed only). Detached bisect gets land_work_untraceable with a runnable git switch; unrelated rebase gets land_rebase_in_progress first for land, merge --check and the Stop hook.
