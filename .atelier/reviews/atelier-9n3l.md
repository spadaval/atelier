---
schema: atelier.review
schema_version: 1
id: atelier-9n3l
title: 'Re-reviewed at d2e7ac1c (fix 720322f4) after findings atelier-bmb4 and atelier-dyke. Checked: human_count now appends '' N context,'' when nonzero for work missions Scope and mission/parent Work lines; quiet scoped summary always carries context=; issue show Buckets adds ''context N,'' when nonzero; is_context reads record.relationships.children via issue_record, the same edge source the index children map is built from, so semantics are unchanged and no records are copied. Rebuilt and confirmed in a scratch repo that mission, parent (quiet and human) and issue show counts add up to scope, with a parent that has only backlog children shown as ''1 context ... backlog 1''; quiet work ready stays IDs only. Ran the targeted test test_work_ready_lists_startable_work_and_shows_parents_and_missions_as_context (pass), which now asserts the summaries; master is an ancestor of HEAD; evidence atelier-uybh records the workspace suite at 720322f4, and d2e7ac1c only adds records. Residual: no-policy mode still never marks children done (pre-existing degradation).'
target: atelier-ymmb
identity: Independent reviewer (Claude subagent)
commit: d2e7ac1c99eedaeda113e6acf57289c54495fc9b
created_at: 2026-10-16T01:06:06.849217646Z
event:
  event: approval
---

Re-reviewed at d2e7ac1c (fix 720322f4) after findings atelier-bmb4 and atelier-dyke. Checked: human_count now appends ' N context,' when nonzero for work missions Scope and mission/parent Work lines; quiet scoped summary always carries context=; issue show Buckets adds 'context N,' when nonzero; is_context reads record.relationships.children via issue_record, the same edge source the index children map is built from, so semantics are unchanged and no records are copied. Rebuilt and confirmed in a scratch repo that mission, parent (quiet and human) and issue show counts add up to scope, with a parent that has only backlog children shown as '1 context ... backlog 1'; quiet work ready stays IDs only. Ran the targeted test test_work_ready_lists_startable_work_and_shows_parents_and_missions_as_context (pass), which now asserts the summaries; master is an ancestor of HEAD; evidence atelier-uybh records the workspace suite at 720322f4, and d2e7ac1c only adds records. Residual: no-policy mode still never marks children done (pre-existing degradation).
