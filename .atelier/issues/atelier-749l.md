---
created_at: "2026-10-15T19:31:01.984457690+00:00"
id: "atelier-749l"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-749l
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:08:16.535975241+00:00"
status: "done"
title: "Name dirty files when land evidence freshness fails"
updated_at: "2026-10-16T19:08:16.535975241+00:00"
---

## Goal

When uncommitted non-record files make done items' passing evidence stale, `atelier land` names the dirty paths instead of only reporting an older failing evidence record.

## Done when

A test with a dirty code file and a done item with an old fail and a newer pass shows a message naming the dirty file.

## Context

Independent validation 2026-10-15: in a clean clone, `echo x > crates/scratch_dirty.rs; atelier land` reports "newer failing evidence atelier-3onc".

## Close Reason

When a newer passing evidence record no longer proves the checkout, evidence.no_newer_failure names it as stale alongside the older failure, and land lists the uncommitted paths (first five, then a count) that make passing proof stale. Test: land_names_uncommitted_files_that_make_a_newer_pass_stale, done child of an active parent with fail then pass evidence and a dirty code file; without the fix it reports only 'newer failing evidence'. The original master reproduction no longer occurs because retained claims are checked without freshness.
