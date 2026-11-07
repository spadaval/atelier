---
schema: atelier.review
schema_version: 2
id: atelier-4t47
title: finding on atelier-4nsl @23dae74d
target: atelier-4nsl
identity: independent reviewer subagent (claude-opus-5-5), atelier-4nsl
commit: 23dae74d3e56985d9c442e71ce68c29a2dc542b4
created_at: 2026-11-07T11:16:49.371522829Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/commands/git.rs:994 (branch_owner_ids at :1233) status's branch list still treats an owner done on the base as owning the branch, so light_row is never reached for it. Done when 7 requires every unowned local branch with branch-target records to be listed with its review state, and the Report's deviation says status follows the new ownership rule, but that holds only for the checked-out branch. Reproduced: issue merged from feat (branch deleted), feat recreated with a new code commit and a branch approval; on feat, status says 'Light path: 2 commits since main, approved'; on main it lists 'feat  t-xxxx done, merged into main', which hides the unmerged commits and their review state. Fix: apply the same owner-done-on-the-base filter (light_path::owners) in branch_summaries_for before choosing the owner rows, and add a status test for that case.
