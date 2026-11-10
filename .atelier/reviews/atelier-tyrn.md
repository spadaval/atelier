---
schema: atelier.review
schema_version: 2
id: atelier-tyrn
title: finding on atelier-fcb7 @8ccc4a45
target: atelier-fcb7
identity: gpt-6-astra-amend2-r2
commit: 8ccc4a45a133d068e47e0cdbd604f3eb1e60ee8e
created_at: 2026-11-10T04:10:24.197112790Z
event:
  event: finding
  recommendation: fix_now
---

medium, docs/strategies/redesign-architecture.md:313-320: the table still cannot deliver the promise that acts committed together appear as separate lines. On the base, ready an existing draft item and close it before committing; planning is committed with Git, and both commands are permitted. The committed status changes from draft to done, so the ready selector never fires, although ready was not undone. The log loses a promised act and its status boundary. Smallest fix: explicitly limit the log to transitions observable between committed snapshots, including omission of intermediate statuses in a batch, and make its exclusions complete (the table also exposes skip trailers only at merge, although other commands write them). Alternatively define detection from retained ready metadata, without promising recovery of unretained transitions. This is the remaining contract gap in atelier-5lh1.
