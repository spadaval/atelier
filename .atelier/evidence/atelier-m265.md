---
created_at: "2026-11-05T02:19:44.758854483+00:00"
id: "atelier-m265"
evidence_type: "validation"
commit: "0a2ec1a757b32aca938657d1f2ee49afdc39a5ec"
result: "pass"
captured_at: "2026-11-05T02:19:44.758436555+00:00"
agent_identity: "Independent acceptance validator (Claude subagent)"
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
title: "S2 PASS on master 0a2ec1a7. Fixture with the work close requiring close_reason plus report.deviations/decisions/surprises/open_items: close with only --reason exits 1 naming all four missing parts and the recovery step (add '### <Part>' or pass --reason-file, 'write None.'); a file with an empty '### Surprises' and one without it each exit 1 naming Surprises; a file with parts but no Outcome exits 1 on close_reason; a full --reason-file closes; a Close Reason written in the record body closes with no flag ('Close Reason: kept from the record'). 'atelier land' lists required_field.report.* among passing checks, and after committing a done claim with its Surprises part removed it refuses: 'fx-7ccf required_field.report.surprises: committed claim is missing required field report.surprises' (exit 1). supersede needs only the Outcome. This repo's .atelier/workflow.yaml requires all four parts on work close; 'atelier check' passes and 'atelier doctor' reports '241 done claims hold' though 114 done records have no Surprises part. Limits: fixture CLI runs plus doctor; the side-branch gap atelier-u0i3 is a documented limit."
updated_at: "2026-11-05T02:19:44.759981450+00:00"
---

S2 PASS on master 0a2ec1a7. Fixture with the work close requiring close_reason plus report.deviations/decisions/surprises/open_items: close with only --reason exits 1 naming all four missing parts and the recovery step (add '### <Part>' or pass --reason-file, 'write None.'); a file with an empty '### Surprises' and one without it each exit 1 naming Surprises; a file with parts but no Outcome exits 1 on close_reason; a full --reason-file closes; a Close Reason written in the record body closes with no flag ('Close Reason: kept from the record'). 'atelier land' lists required_field.report.* among passing checks, and after committing a done claim with its Surprises part removed it refuses: 'fx-7ccf required_field.report.surprises: committed claim is missing required field report.surprises' (exit 1). supersede needs only the Outcome. This repo's .atelier/workflow.yaml requires all four parts on work close; 'atelier check' passes and 'atelier doctor' reports '241 done claims hold' though 114 done records have no Surprises part. Limits: fixture CLI runs plus doctor; the side-branch gap atelier-u0i3 is a documented limit.
