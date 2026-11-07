---
schema: atelier.review
schema_version: 2
id: atelier-lezj
title: finding on atelier-4nsl @23dae74d
target: atelier-4nsl
identity: independent reviewer subagent (claude-opus-5-5), atelier-4nsl
commit: 23dae74d3e56985d9c442e71ce68c29a2dc542b4
created_at: 2026-11-07T11:16:41.912401326Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/light_path.rs:356-371 owned_when_merged finds the branch's owners with git grep -F for 'work_branch: <name>' or 'work_branch: "<name>"', but issue records write workflow_branch through serde_yaml, which single-quotes names that read as numbers, booleans or null (e.g. work_branch: '1.0'). For such a branch, land on the base counts a pre-start branch approval as covering an owned branch's commits, which the hard constraint and ADR 0028 decision 6 ('the branch was not owned when it merged') forbid. Reproduced in a scratch repo: branch 1.0 gets a branch approval, atelier start takes it, a plain git merge into main, then land passes; the same steps with branch foo refuse with land_base_unreviewed. Fix: match quoted forms too (or grep for work_branch: and compare the parsed value, as the loop already does), and add a test with a YAML-quoted branch name.
