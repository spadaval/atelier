---
schema: atelier.review
schema_version: 1
id: atelier-hu66
title: 'crates/atelier-app/src/objective_graph.rs (dependency_closure, inherited branch): a real deadlock through the parent is reported as an incomplete path, not a cycle. Repro (/tmp/e4c5s3): P blocked_by B (B outside P), K --parent P, then B blocked_by K. issue transition K reads ''transitive blocker path K -> P -> B -> K'' rather than ''dependency cycle ... repair the declared blocked_by relationships''. The parent walk starts with path [P], so the child is not on the path when the walk returns to it. Readiness is still correct (K and B both refuse start), so only the wording and the repair hint are lost. Fix: start the inherited walk with path [K, P], or treat a return to issue_id as a cycle.'
target: atelier-e4c5
identity: Independent reviewer (Claude subagent)
commit: 5b0ab3a3537f4297187238df94641fa10a9eaedd
created_at: 2026-10-16T00:55:52.411458189Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/objective_graph.rs (dependency_closure, inherited branch): a real deadlock through the parent is reported as an incomplete path, not a cycle. Repro (/tmp/e4c5s3): P blocked_by B (B outside P), K --parent P, then B blocked_by K. issue transition K reads 'transitive blocker path K -> P -> B -> K' rather than 'dependency cycle ... repair the declared blocked_by relationships'. The parent walk starts with path [P], so the child is not on the path when the walk returns to it. Readiness is still correct (K and B both refuse start), so only the wording and the repair hint are lost. Fix: start the inherited walk with path [K, P], or treat a return to issue_id as a cycle.
