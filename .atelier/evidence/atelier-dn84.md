---
created_at: "2026-11-05T04:40:44.582971797+00:00"
id: "atelier-dn84"
evidence_type: "validation"
commit: "4f712c5d80bf673bae94279c566fce31ddd1f352"
result: "fail"
captured_at: "2026-11-05T04:40:44.582904912+00:00"
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
title: "S4 FAIL on debrief code at c73d55e8 (same code as this commit). Holds: criterion marks with linked proof; done children's reports in both forms (all 27 child report parts of r6bt found in the HTML); all 47 r6bt decision notes where they were made, in both forms; work decisions lists records then notes; derived deviations; follow-ups; one view model, self-contained HTML; 23 debrief fixtures pass. FAILS 'each item's report': the report's ### Decisions part is never rendered, so 9 vt8q items' decisions (e.g. Sumeet's v0.3.0 over v0.2.0 in vy5i) appear in neither form. Fix: atelier-mgl0."
updated_at: "2026-11-05T04:40:44.583526099+00:00"
---

S4 FAIL on debrief code at c73d55e8 (same code as this commit). Holds: criterion marks with linked proof; done children's reports in both forms (all 27 child report parts of r6bt found in the HTML); all 47 r6bt decision notes where they were made, in both forms; work decisions lists records then notes; derived deviations; follow-ups; one view model, self-contained HTML; 23 debrief fixtures pass. FAILS 'each item's report': the report's ### Decisions part is never rendered, so 9 vt8q items' decisions (e.g. Sumeet's v0.3.0 over v0.2.0 in vy5i) appear in neither form. Fix: atelier-mgl0.
