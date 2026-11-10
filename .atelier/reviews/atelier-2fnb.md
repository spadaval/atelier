---
schema: atelier.review
schema_version: 2
id: atelier-2fnb
title: finding on atelier-295u @aa0dc513
target: atelier-295u
identity: gpt-6-astra-amend2-r3
commit: aa0dc513044850193e7de9f6993b7ed5aa03cc5a
created_at: 2026-11-10T04:12:17.827925293Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 2187abdb2222bcd72ab0edb299893533c0ab637b
    recorded_at: 2026-11-10T04:12:41.174132848Z
    reason: The introduction is corrected in 2187abdb although recommended defer; timing coverage stays with the log step's planning.
---

low, docs/strategies/redesign-4-gardener.md:16-18: the introduction still says every act is a commit that changes stored status, although lead, escalate and decide preserve status and planning acts can share a commit. This overstates the measurement coverage, but the architecture now explicitly limits log to committed net changes and the from-log criterion excludes draft time, resolving the substantive atelier-ecmi contract gap. Defer to atelier-8b0a planning: replace that sentence with a statement about observable record changes in Git, and specify how timing coverage reports omitted intermediate statuses rather than inventing durations.
