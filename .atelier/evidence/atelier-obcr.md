---
created_at: "2026-11-05T02:19:57.599861598+00:00"
id: "atelier-obcr"
evidence_type: "validation"
commit: "0a2ec1a757b32aca938657d1f2ee49afdc39a5ec"
result: "fail"
captured_at: "2026-11-05T02:19:57.599448840+00:00"
agent_identity: "Independent acceptance validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S4"
  result: "fail"
criteria_mission: "atelier-r6bt"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r6bt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S4 FAIL on master 0a2ec1a7 (defect in this change). Holds: fixture mission (plan approval, decisions with ADR and supersedes, decision notes on item and mission, scope added after start, dropped item with reason, deferred finding with follow-up, trivial base exemption, reopened item, S1 pass/S2 fail receipts then code commits) gives a debrief whose Markdown and --html show each criterion mark with receipt, commit and code commits since (identical to 'work mission'), outcomes, Deviations/Surprises of top-level items, derived Changed lines (Reopened on ...), the trivial exemption, dropped work with why, the deferred finding and Filed follow-up, plan approval, Needs Your Review; HTML has no script/external resource, a prefers-color-scheme palette, and is gitignored. On a branch that merged master the timeline equals master's. Real runs: 'debrief atelier-vt8q' (1.3s) and --html succeed; all 29 IDs in the Markdown appear in the HTML. FAILS: (1) A done child's report is not shown. Fixture child fx-7ccf's Deviations ('Added a warmup step') and Surprises ('memoised fonts') appear in neither form, its Outcome only in the HTML card; in the real 'debrief atelier-r6bt', atelier-w5hx's Surprises ('work decisions -q also changed order') and Open items appear nowhere, w5hx only as a child line under atelier-2bjz. The Markdown renders the entry's own report only (commands/mission_review.rs render of WorkEntry: children as item lines). (2) Neither form has a decision log with decision records first; decisions sit in timeline entries, by design since atelier-8bil, which left 'whether that still reads as S4' to the human; S4's 2026-11-04 rewording kept the decision-log wording. (3) Minor: the report's '### Decisions' part text is never rendered (decision notes replace it). Repro: in a repo whose policy requires report parts, close a child with non-None Deviations/Surprises under a done parent, run 'atelier debrief <mission>'; expected the child's report, actual a '└ <child>' line. Limits: no fixture tests run locally."
updated_at: "2026-11-05T02:19:57.601669332+00:00"
---

S4 FAIL on master 0a2ec1a7 (defect in this change). Holds: fixture mission (plan approval, decisions with ADR and supersedes, decision notes on item and mission, scope added after start, dropped item with reason, deferred finding with follow-up, trivial base exemption, reopened item, S1 pass/S2 fail receipts then code commits) gives a debrief whose Markdown and --html show each criterion mark with receipt, commit and code commits since (identical to 'work mission'), outcomes, Deviations/Surprises of top-level items, derived Changed lines (Reopened on ...), the trivial exemption, dropped work with why, the deferred finding and Filed follow-up, plan approval, Needs Your Review; HTML has no script/external resource, a prefers-color-scheme palette, and is gitignored. On a branch that merged master the timeline equals master's. Real runs: 'debrief atelier-vt8q' (1.3s) and --html succeed; all 29 IDs in the Markdown appear in the HTML. FAILS: (1) A done child's report is not shown. Fixture child fx-7ccf's Deviations ('Added a warmup step') and Surprises ('memoised fonts') appear in neither form, its Outcome only in the HTML card; in the real 'debrief atelier-r6bt', atelier-w5hx's Surprises ('work decisions -q also changed order') and Open items appear nowhere, w5hx only as a child line under atelier-2bjz. The Markdown renders the entry's own report only (commands/mission_review.rs render of WorkEntry: children as item lines). (2) Neither form has a decision log with decision records first; decisions sit in timeline entries, by design since atelier-8bil, which left 'whether that still reads as S4' to the human; S4's 2026-11-04 rewording kept the decision-log wording. (3) Minor: the report's '### Decisions' part text is never rendered (decision notes replace it). Repro: in a repo whose policy requires report parts, close a child with non-None Deviations/Surprises under a done parent, run 'atelier debrief <mission>'; expected the child's report, actual a '└ <child>' line. Limits: no fixture tests run locally.
