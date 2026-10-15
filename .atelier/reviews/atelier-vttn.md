---
schema: atelier.review
schema_version: 1
id: atelier-vttn
title: 'Low: record_index.rs:462-468 skips an edge rejected for multiple parents before validate_hierarchy_shapes sees it. With epic P1 and task P2 both listing child C, check reports only ''C appears under multiple parents: P1, P2''. The second edge''s own ''cannot be child of task P2'' error appears only after the first is fixed. This is the defined granularity of one error per invalid edge, the same as the old code. Recommend NO ACTION.'
target: atelier-1ioo
identity: independent-reviewer-3-2026-10-15
commit: 8f13d015eda0d0d702e77bf318fb686565e62955
created_at: 2026-10-15T19:30:04.089345349Z
event:
  event: finding
  recommendation: no_action
---

Low: record_index.rs:462-468 skips an edge rejected for multiple parents before validate_hierarchy_shapes sees it. With epic P1 and task P2 both listing child C, check reports only 'C appears under multiple parents: P1, P2'. The second edge's own 'cannot be child of task P2' error appears only after the first is fixed. This is the defined granularity of one error per invalid edge, the same as the old code. Recommend NO ACTION.
