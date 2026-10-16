---
schema: atelier.review
schema_version: 1
id: atelier-ul3c
title: 'crates/atelier-app/src/workflow_policy.rs:33-82 (validate_issue_hierarchy), regression. The children check now runs only inside ''if let Some(parent_id)'', so nothing stops a parent with children from being retyped to mission. Repro in a fresh init repo: create task P with children C1 and C2, then ''atelier issue update P --issue-type mission''. It succeeds and writes a mission record that still lists children. After that every command, including check, issue update and work, fails while loading the index (''issue C cannot be child of mission P''), so the repo is stuck until someone edits the files by hand. Master rejected this (''cannot own child work''). Fix: when issue.issue_type == mission and index.get_subissues(&issue.id) is not empty, return workflow_issue_hierarchy_invalid. Add a CLI test for update --issue-type mission on a parent.'
target: atelier-o6h9
identity: Independent reviewer (Claude subagent)
commit: 4f66aab9fee9062a0f8cf6b3f586acc04a0cc218
created_at: 2026-10-16T00:37:30.847133608Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/workflow_policy.rs:33-82 (validate_issue_hierarchy), regression. The children check now runs only inside 'if let Some(parent_id)', so nothing stops a parent with children from being retyped to mission. Repro in a fresh init repo: create task P with children C1 and C2, then 'atelier issue update P --issue-type mission'. It succeeds and writes a mission record that still lists children. After that every command, including check, issue update and work, fails while loading the index ('issue C cannot be child of mission P'), so the repo is stuck until someone edits the files by hand. Master rejected this ('cannot own child work'). Fix: when issue.issue_type == mission and index.get_subissues(&issue.id) is not empty, return workflow_issue_hierarchy_invalid. Add a CLI test for update --issue-type mission on a parent.
