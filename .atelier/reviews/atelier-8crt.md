---
schema: atelier.review
schema_version: 1
id: atelier-8crt
title: 'Low: record_index.rs:810 still runs the mission-dependency policy check on a graph with a blocks cycle. When an in_progress or ready mission is blocked only by done issues that form a cycle, check reports ''relationships.blocks contains a cycle'' and also ''workflow_mission_dependency_bypass: mission M ... despite dependency cycle(s): M -> A -> B -> A''. The second error cascades from the first, but it is a true statement and names the cycle path, which the bare cycle error does not. Recommend NO ACTION.'
target: atelier-1ioo
identity: independent-reviewer-3-2026-10-15
commit: 8f13d015eda0d0d702e77bf318fb686565e62955
created_at: 2026-10-15T19:30:04.051270943Z
event:
  event: finding
  recommendation: no_action
---

Low: record_index.rs:810 still runs the mission-dependency policy check on a graph with a blocks cycle. When an in_progress or ready mission is blocked only by done issues that form a cycle, check reports 'relationships.blocks contains a cycle' and also 'workflow_mission_dependency_bypass: mission M ... despite dependency cycle(s): M -> A -> B -> A'. The second error cascades from the first, but it is a true statement and names the cycle path, which the bare cycle error does not. Recommend NO ACTION.
