---
created_at: "2026-11-11T02:20:12.349652625+00:00"
id: "atelier-hgve"
evidence_type: "validation"
commit: "df6d8f3f60978b0971f7eeac9ea447aee91dcc1b"
result: "pass"
captured_at: "2026-11-11T02:20:12.349557397+00:00"
agent_identity: "claude-fable-5-2-orchestrator"
target:
  kind: "issue"
  id: "atelier-fcb7"
  role: "validates"
criteria:
- slug: "rehearsal"
  result: "pass"
criteria_mission: "atelier-fcb7"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fcb7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "On master 476f8ee4, scripts/cutover/rehearse.sh run from fresh scratch clones with the binary built from that commit: for each of atelier, astromech, demiurge, life-on-rails and cosplay-studio the converter ran, convert_records.py --verify passed, the new binary's check found every record valid (162, 240, 19, 47 and 37 items), setup installed the three Git hooks and the pre-commit hook refused a broken record. One item went from ready to done by the commands alone in the atelier clone (3 merge checks ran) and in the life-on-rails clone (1 merge check), with the forge path against a gh double and pushes to local bare repositories. All steps passed in 34 seconds. status and list took at most 0.57 seconds, on the largest repository with 31 local branches. The five real repositories were unchanged before and after."
updated_at: "2026-11-11T02:20:12.509220874+00:00"
---

On master 476f8ee4, scripts/cutover/rehearse.sh run from fresh scratch clones with the binary built from that commit: for each of atelier, astromech, demiurge, life-on-rails and cosplay-studio the converter ran, convert_records.py --verify passed, the new binary's check found every record valid (162, 240, 19, 47 and 37 items), setup installed the three Git hooks and the pre-commit hook refused a broken record. One item went from ready to done by the commands alone in the atelier clone (3 merge checks ran) and in the life-on-rails clone (1 merge check), with the forge path against a gh double and pushes to local bare repositories. All steps passed in 34 seconds. status and list took at most 0.57 seconds, on the largest repository with 31 local branches. The five real repositories were unchanged before and after.
