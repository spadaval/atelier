# Adaptive Mission Gap Analysis

This audit maps the adaptive Agent Factory strategy to observable Atelier
behavior. It is the planning evidence for `atelier-js22`, not a second backlog.
The governing design is
[ADR 0018](../../adr/0018-strategy-draft-epics-and-adaptive-expansion.md).

| Operator scenario | Existing capability to reuse | Gap found | Smallest change |
| --- | --- | --- | --- |
| Find the governing strategy | Missions are canonical issue records and ordinary Markdown can be versioned beside them. | The strategy path and revision lived only in prose. | Add one typed, optional, mission-only `strategy: {path, revision}` reference and show it in the focused mission dashboard. |
| Keep future outcomes visible without claiming they are executable | Epic records already own branch, review, hierarchy, and parent validation boundaries. | The epic workflow began at `todo`, and queues equated the whole `todo` category with ready work. | Begin epics at `draft`, refine them to `ready`, and derive ready eligibility from a direct transition into active work plus dependency readiness. |
| Expand only the next evidence-backed increment | Issue sections, child links, blocker links, and workflow transitions already describe the graph. | No lifecycle boundary distinguished a provisional outcome from an executable epic. | Define expansion as ordinary refinement plus child/dependency creation and the `draft -> ready` transition. Do not add an expansion record or flag. |
| Choose proof when the increment is understood | Evidence sections are optional; evidence records, validators, and validation issues already support risk-scaled proof. | Bundle import synthesized meaningless Evidence placeholders, and guidance implied proof must always be fixed during distant planning. | Omit empty bundle Evidence sections and require proof-route selection when an epic becomes ready, not while it remains draft. |
| Replan within strategy authority | Managers can create and update issues, edit dependencies and hierarchy, add notes, and preserve evidence and Git history. | Obsolete speculative epics had no clean terminal path in the epic workflow. | Add an epic `supersede` transition with a close reason. Use direct graph edits and an optional mission note for rationale. |
| See outcome map and execution horizon together | Focused mission dashboards already group mission-scoped work. | Draft epics appeared in Ready Work because category was treated as readiness. | Classify non-startable todo-category states as backlog/context and retain exact `draft` status in the row. |
| Keep portable orchestration separate from repository mechanics | Agent Factory routes tracker operations through a workspace binding. | Guidance still assumes a substantially pre-expanded plan and does not name draft-to-ready proof planning. | Update portable orchestration principles and the Atelier binding after the repository contract and behavior settle. |

## Machinery Rejected

The scenarios do not justify outcome anchors, a second epic type, validation
profiles, work dispositions, a replanning record, plan approval, actor
authorization, graph digests, or whole-mission completeness gates. Workflow
state, authored outcomes, typed relationships, evidence, notes, and Git history
already own the required facts.

## Remaining Evidence Horizon

The current contract epic can close after focused tests cover the typed strategy
round trip and display, draft/ready queue classification, relaxed bundle output,
and workflow transitions, followed by independent red-line validation. The
later presentation, replanning, and Agent Factory epics remain draft until that
evidence justifies their exact work and proof routes.
