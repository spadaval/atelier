---
schema: atelier.review
schema_version: 2
id: atelier-m9li
title: finding on atelier-wd1i @d41d3bcd
target: atelier-wd1i
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:52:17.758881708Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-wd1i.md:45-46 Quiet first requires no item in progress before merging the switch, but atelier-u1ko is ordinary work started on its branch and only atelier merge marks it done (AGENTS.md's One issue order; crates/atelier-cli/src/commands/start.rs and merge.rs). A switch held approved and unmerged is consequently still in_progress in the old tool. The stated premerge condition can never hold on the intended happy path, so an executor must invent an exception or mark the switch done outside its merge. Define the premerge check as no in-progress work except this exact approved switch, with its worker/review threads quiescent; then require no in-progress work at the script's postmerge preflight. Record this narrow exception in the reviewed runbook/protocol and exercise it without prematurely changing the switch's status.
