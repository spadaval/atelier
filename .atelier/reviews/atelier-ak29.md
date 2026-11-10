---
schema: atelier.review
schema_version: 2
id: atelier-ak29
title: finding on atelier-yptz @3cb4e377
target: atelier-yptz
identity: gpt-6.1-sol-r1
commit: 3cb4e3779f9d8b7d862aebc8e751cffa3f714b73
created_at: 2026-11-10T20:12:34.980287784Z
event:
  event: finding
  recommendation: fix_now
---

R1, low severity: next/crates/atelier-cli/src/output.rs:457 joins planning paths into a shell command without quoting them (and line 454 leaves the checkout path unquoted). Mission directories explicitly accept arbitrary planning documents. Reproduced at the pinned head in a scratch repository: commit a draft mission and item, ready both without committing, add missions/<id>/design notes.md, and run start <item>. Its Commit the planning with command exits 128: pathspec .atelier/missions/<id>/design did not match any files. The same command is printed by status. Shell metacharacters in a document or checkout name are also interpreted as shell syntax instead of filename bytes. Shell-quote each path and the git -C argument; exercise the printed command with a spaced document name and a spaced base checkout path. Recommendation: fix_now, because the new uncommitted-readiness refusal does not give a working way out for supported mission documents.
