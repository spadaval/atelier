---
created_at: "2026-10-17T20:09:29.428107477+00:00"
id: "atelier-qkk8"
evidence_type: "validation"
commit: "0edf25bb740431c17e96c6248e56d22c693eb968"
result: "pass"
captured_at: "2026-10-17T20:09:29.428021026+00:00"
agent_identity: "Independent validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S2"
  result: "pass"
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
title: "S2 PASS on master 0edf25bb. This repo's .atelier/workflow.yaml work.close requires close_reason plus report.deviations/decisions/surprises/open_items; supersede needs only close_reason. Scratch repo using that policy: close with Outcome only is refused listing the four missing parts with a recovery step (add ### sections or --reason-file); an empty ### Surprises and a missing one are refused naming Surprises; a report without Outcome is refused for close_reason; full --reason-file closes. A committed claim edited to drop ### Surprises makes 'atelier land' (and doctor) fail with required_field.report.surprises and a recovery step; supersede with --reason alone succeeded. Existing records: in the real repo 110 done records lack ### Surprises, yet 'atelier check', 'atelier doctor' and 'atelier land' all pass on master. Suites 802/802 and 806/806 (reports::*, session::report_parts_bind_new_done_claims_and_leave_earlier_claims_valid, report_parts_leave_scoped_claims_made_before_the_policy_valid). Limit: did not hand-craft a pre-policy claim in scratch; relied on the real repo's history and tests for that."
updated_at: "2026-10-17T20:09:29.428734457+00:00"
---

S2 PASS on master 0edf25bb. This repo's .atelier/workflow.yaml work.close requires close_reason plus report.deviations/decisions/surprises/open_items; supersede needs only close_reason. Scratch repo using that policy: close with Outcome only is refused listing the four missing parts with a recovery step (add ### sections or --reason-file); an empty ### Surprises and a missing one are refused naming Surprises; a report without Outcome is refused for close_reason; full --reason-file closes. A committed claim edited to drop ### Surprises makes 'atelier land' (and doctor) fail with required_field.report.surprises and a recovery step; supersede with --reason alone succeeded. Existing records: in the real repo 110 done records lack ### Surprises, yet 'atelier check', 'atelier doctor' and 'atelier land' all pass on master. Suites 802/802 and 806/806 (reports::*, session::report_parts_bind_new_done_claims_and_leave_earlier_claims_valid, report_parts_leave_scoped_claims_made_before_the_policy_valid). Limit: did not hand-craft a pre-policy claim in scratch; relied on the real repo's history and tests for that.
