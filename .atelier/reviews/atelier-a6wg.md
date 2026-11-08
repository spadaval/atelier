---
schema: atelier.review
schema_version: 2
id: atelier-a6wg
title: finding on atelier-ss4y @623aed9f
target: atelier-ss4y
identity: grok-plan-reviewer-ss4y (grok-4.6)
commit: 623aed9f79127a2c7512dd990f9899a445519a6a
created_at: 2026-11-08T21:52:42.511464888Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: grok-plan-rereviewer-ss4y (grok-4.6)
    commit: 7e009e0ec8a382e414e03845f3c703f0ba5935ea
    recorded_at: 2026-11-08T22:00:50.340854724Z
    reason: 7e009e0e replaced the sidebar-nesting Done when with a Goal claim citing resolveSidebarRowThreadId. T3 hides subagent threads from the sidebar and shows them under the nearest listed ancestor, so blanket delegate_task does not add sidebar rows.
---

Low complexity, medium value, low risk. ixdi's sidebar-nesting check is an evidence boundary mixed into an implementation task. plan.md wants that as a spike or decision that blocks the work. The Done when already says check first and report before landing, so it need not block this planning edit; splitting it still keeps a 1,700-thread-a-month product question from hiding inside a procedure change. If crowding is unacceptable, do not land blanket delegate_task: only roles that must receive t3_thread_send (implementers) need to be T3 threads. Native one-shot reviews can stay on Agent.
