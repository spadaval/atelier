---
created_at: "2026-10-17T20:09:45.093504558+00:00"
id: "atelier-82x5"
evidence_type: "validation"
commit: "0edf25bb740431c17e96c6248e56d22c693eb968"
result: "pass"
captured_at: "2026-10-17T20:09:45.093397067+00:00"
agent_identity: "Independent validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S6"
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
title: "S6 PASS on master 0edf25bb. Scratch: decision notes on child C (reversible), its parent P (one-way), the mission (reversible), and a sibling (one-way); mission records D1 superseded by D2. 'issue show C' brief: 'Decisions: 4 current, 1 one-way:' listing only P's one-way note; 'issue show C --full' Decisions section lists all 4 (one-way first, current record D2, C's and mission's notes), excluding the sibling's note and superseded D1. 'atelier status' after starting work (owner became parent P): 'Decisions: 3 current, 1 one-way:' with P's one-way note. Limit: status has no full view; the full listing is 'issue show --full'. Tests decisions::issue_show_lists_the_current_decisions_bearing_on_an_item and session::status_shows_the_owner_decisions_in_brief pass."
updated_at: "2026-10-17T20:09:45.094210595+00:00"
---

S6 PASS on master 0edf25bb. Scratch: decision notes on child C (reversible), its parent P (one-way), the mission (reversible), and a sibling (one-way); mission records D1 superseded by D2. 'issue show C' brief: 'Decisions: 4 current, 1 one-way:' listing only P's one-way note; 'issue show C --full' Decisions section lists all 4 (one-way first, current record D2, C's and mission's notes), excluding the sibling's note and superseded D1. 'atelier status' after starting work (owner became parent P): 'Decisions: 3 current, 1 one-way:' with P's one-way note. Limit: status has no full view; the full listing is 'issue show --full'. Tests decisions::issue_show_lists_the_current_decisions_bearing_on_an_item and session::status_shows_the_owner_decisions_in_brief pass.
