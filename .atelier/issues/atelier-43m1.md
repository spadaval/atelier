---
created_at: "2026-10-17T00:33:21.238986900+00:00"
id: "atelier-43m1"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Define Close Reason's expected content in work-model.md"
updated_at: "2026-10-17T00:33:55.956122067+00:00"
---

## Goal

docs/product/work-model.md said only that Close Reason carries "terminal reasons". Agents wrote 10 to 118 word result summaries with test names and review status in it. State what belongs there so the S2 report in atelier-r6bt extends a defined field.

## Done when

work-model.md defines Close Reason as one or two sentences giving the observable result against Done when, or for supersede why the work will not happen, and says test names, measurements and review status belong in evidence and review records.

## Context

Committed on master in 1620b534 by human direction on 2026-10-17 (prose-only documentation). The land gate refused the commit as unreviewed and the push ran anyway because the shell pipeline took tail's exit status; this record and its exemption make that visible.

## Notes

- 2026-10-17 [handoff] 2026-10-17: the doc edit is already on master (1620b534, trivial exemption atelier-36rk). Nothing left to write; the item exists so the base commit is covered. Close it from a work branch when convenient, since done claims may not be made on master.
