---
schema: atelier.review
schema_version: 2
id: atelier-t4hu
title: finding on atelier-1sbw @82f0b9b6
target: atelier-1sbw
identity: gpt-6.1-sol-r1
commit: 82f0b9b6acae40566f4062ec4abab31875eb1c30
created_at: 2026-11-10T23:05:18.738212318Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:668 R1 --verify omits ignored files from its comparison because git ls-files uses --exclude-standard. On the invented repository, add /.atelier/runtime/ to .gitignore in the old commit, convert, then create .atelier/runtime/leftover.json: --verify <old> exits 0 and claims the tree matches. This repository already ignores /.atelier/runtime/, so the verification can silently accept an old-layout directory that the contract requires to be absent. Enumerate every file under .atelier independently of Git ignore rules, and test that an ignored extra file makes --verify exit nonzero and name that file.
