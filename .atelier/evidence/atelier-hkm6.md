---
created_at: "2026-11-05T02:20:11.941885736+00:00"
id: "atelier-hkm6"
evidence_type: "validation"
commit: "0a2ec1a757b32aca938657d1f2ee49afdc39a5ec"
result: "pass"
captured_at: "2026-11-05T02:20:11.941499477+00:00"
agent_identity: "Independent acceptance validator (Claude subagent)"
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
title: "S6 PASS on master 0a2ec1a7. Fixture: decision note on parent T1 and on the mission, child T3 of T1, decided records D2 (supersedes D1), D4, D5, D6, open D3. 'issue show T3' brief and 'atelier status' (owner T1) print 'Decisions: 6 current:' with the three newest records, then '... 1 more decision record omitted; see atelier issue show <id> --full'. 'issue show T3 --full' Decisions lists all 6: the four current records (D2 with '| supersedes D1') then T1's and the mission's notes with because/rejected; superseded D1 and open D3 are excluded. Limits: CLI fixture runs; status has no full view (the full list is issue show --full)."
updated_at: "2026-11-05T02:20:11.943183671+00:00"
---

S6 PASS on master 0a2ec1a7. Fixture: decision note on parent T1 and on the mission, child T3 of T1, decided records D2 (supersedes D1), D4, D5, D6, open D3. 'issue show T3' brief and 'atelier status' (owner T1) print 'Decisions: 6 current:' with the three newest records, then '... 1 more decision record omitted; see atelier issue show <id> --full'. 'issue show T3 --full' Decisions lists all 6: the four current records (D2 with '| supersedes D1') then T1's and the mission's notes with because/rejected; superseded D1 and open D3 are excluded. Limits: CLI fixture runs; status has no full view (the full list is issue show --full).
