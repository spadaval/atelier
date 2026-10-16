---
created_at: "2026-10-15T22:08:59.015438310+00:00"
id: "atelier-r7cd"
issue_type: "task"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: task/atelier-r7cd
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-by8u"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T01:30:45.223529488+00:00"
status: "done"
title: "Decide what pruning keeps"
updated_at: "2026-10-16T01:30:45.223529488+00:00"
---

## Goal

Prune removes terminal records after 7 days. That's fine for routine work, but research, decisions and references attached to closed work then exist only in Git history, which weakens "records hold everything". It's also unverified what happens to live links that point at a pruned record.

## Done when

Sumeet decides what survives pruning (for example records with a Decision or research section, or anything linked from an open record, or a per-record keep flag), the rule is implemented, and `check` handles links to pruned records with a test.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Related: atelier-by8u (a place for research and references).

## Notes

- 2026-10-16 [human] Sumeet chose: pruning keeps records linked from an open record (already true: prune keeps any record with a link to or from a retained record) and records with decision or research content (decision records and any record with a Decision or Background section). check names the commit that pruned a missing link target and how to restore it.

## Close Reason

Decided by Sumeet 2026-10-15: pruning keeps records linked from an open record (prune already keeps any record linked to or from a retained one) and records with decision or research content (decision records, Decision or Background sections). A missing reference to a pruned record now names the deleting commit and the git restore command that brings it back. Documented in docs/product/retention-and-prune.md. Test: test_prune_keeps_decision_and_research_records_and_check_explains_pruned_links.
