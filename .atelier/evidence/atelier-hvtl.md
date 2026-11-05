---
created_at: "2026-11-05T02:19:45.734297476+00:00"
id: "atelier-hvtl"
evidence_type: "validation"
commit: "0a2ec1a757b32aca938657d1f2ee49afdc39a5ec"
result: "pass"
captured_at: "2026-11-05T02:19:45.733808486+00:00"
agent_identity: "Independent acceptance validator (Claude subagent)"
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
title: "S3 PASS on master 0a2ec1a7. Fixture: 'issue update <decision> --adr docs/adr/9999-missing.md' exits 1 ('write the ADR first'); an existing path links; a hand-edited missing path makes 'check' report 'Decision ADR ... does not exist'. D2 'supersedes' D1: while D2 is open, 'work decisions' keeps D1 as a current record and lists D2 under Open Decision Records with '| supersedes'; once D2 is decided, D1 moves to Superseded Decisions with '| ADR ... | superseded by D2', 'issue show D1' prints 'Superseded by: D2', and the current-decision counts in issue show/status exclude D1. The debrief shows D1's ADR and 'Replaces: D1' on D2. Limits: CLI fixture runs, not the repo's tests; the debrief timeline does not itself mark D1 as superseded (only D2's Replaces line)."
updated_at: "2026-11-05T02:19:45.737672377+00:00"
---

S3 PASS on master 0a2ec1a7. Fixture: 'issue update <decision> --adr docs/adr/9999-missing.md' exits 1 ('write the ADR first'); an existing path links; a hand-edited missing path makes 'check' report 'Decision ADR ... does not exist'. D2 'supersedes' D1: while D2 is open, 'work decisions' keeps D1 as a current record and lists D2 under Open Decision Records with '| supersedes'; once D2 is decided, D1 moves to Superseded Decisions with '| ADR ... | superseded by D2', 'issue show D1' prints 'Superseded by: D2', and the current-decision counts in issue show/status exclude D1. The debrief shows D1's ADR and 'Replaces: D1' on D2. Limits: CLI fixture runs, not the repo's tests; the debrief timeline does not itself mark D1 as superseded (only D2's Replaces line).
