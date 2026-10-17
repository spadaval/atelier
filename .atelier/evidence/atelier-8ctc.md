---
created_at: "2026-10-17T20:09:29.496561288+00:00"
id: "atelier-8ctc"
evidence_type: "validation"
commit: "0edf25bb740431c17e96c6248e56d22c693eb968"
result: "pass"
captured_at: "2026-10-17T20:09:29.496508790+00:00"
agent_identity: "Independent validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S3"
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
title: "S3 PASS on master 0edf25bb. Scratch: 'issue update <decision> --adr docs/adr/nope.md' refused (file missing); --adr on a task refused; ADR linked on decision D2; 'issue link D2 D1 --role supersedes'. While D2 open, work decisions shows D1 under Decision Records and D2 under Open Decision Records with 'ADR ... | supersedes D1'. After deciding D2: D2 is the current record, D1 under Superseded Decisions 'superseded by D2'; 'work decisions -q' lists only current ones; 'issue show D1' shows 'Superseded by:', 'issue show D2 --full' shows ADR and Supersedes; deleting the ADR file makes check fail. Test decisions::decision_records_link_an_adr_and_show_supersession passes in both suites."
updated_at: "2026-10-17T20:09:29.496924485+00:00"
---

S3 PASS on master 0edf25bb. Scratch: 'issue update <decision> --adr docs/adr/nope.md' refused (file missing); --adr on a task refused; ADR linked on decision D2; 'issue link D2 D1 --role supersedes'. While D2 open, work decisions shows D1 under Decision Records and D2 under Open Decision Records with 'ADR ... | supersedes D1'. After deciding D2: D2 is the current record, D1 under Superseded Decisions 'superseded by D2'; 'work decisions -q' lists only current ones; 'issue show D1' shows 'Superseded by:', 'issue show D2 --full' shows ADR and Supersedes; deleting the ADR file makes check fail. Test decisions::decision_records_link_an_adr_and_show_supersession passes in both suites.
