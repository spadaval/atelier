---
schema: atelier.review
schema_version: 2
id: atelier-icpp
title: finding on atelier-a4rs @17f6947d
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: 17f6947d4cbe2556a869f3c191bf447b52836c23
created_at: 2026-11-11T06:10:38.683474520Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-decide/grade.py:52-57 excludes the working item.md whenever another branch is checked out, while line 70 exempts ITEM from other_files_changed. Reproduced with the copied 399fca6a binary: decide slug-k3fa with --defer, commit the answer, switch the scratch fixture to main, and replace its Scope with Any file including urls.py. The grader returns pass=true with stray_item_changes={} and other_files_changed=[], despite the dirty Scope change. This leaves the item-record part of the branch-switch blind spot after atelier-4w44. Check working item.md against its checked-out baseline on every branch, preserving legitimate unchanged main records, and add a failing walk for off-branch item edits.
