---
schema: atelier.review
schema_version: 2
id: atelier-pbz5
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:51:50.451423599Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:75-76 The rollback proof requires every clone's base to equal its pretrial base after every trial, but trial 5 has already pushed atelier's conversion. The prescribed non-force rollback creates and pushes a revert; its base commit cannot equal the recorded pre-conversion commit, even when the rollback is correct. The same applies to the first product repository at trial 6. This Done when cannot pass under Transition's Rollback table without resetting published history, which is forbidden. Distinguish unpushed cases (exact recorded base commit) from pushed cases (a revert history whose tracked tree equals the recorded tree, with local/remote tips agreeing), and assert the old records, hooks and fallback work in both. Give G the same tree-versus-commit distinction for its rollback checks.
