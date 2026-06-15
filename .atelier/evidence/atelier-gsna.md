---
created_at: "2026-06-13T04:19:24.390734389+00:00"
id: "atelier-gsna"
evidence_type: "validation"
captured_at: "2026-06-13T04:19:24.390702993+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0vjq"
    role: "validates"
  - kind: "issue"
    id: "atelier-s8z0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Docs contract defines unified evidence record workflow: evidence record replaces split add/capture operator choice; target syntax is issue/<id>; command transcript behavior preserves command, exit status, success, timestamp, and bounded stdout/stderr; structured schema covers targets, proof_scope, kind, result, commands/artifacts, agent identity, independence_level, residual_risks, and follow_up_ids; parent coverage summaries and legacy prose migration expectations documented. Checks passed: git diff --check -- '*.md'; atelier lint; atelier export --check."
updated_at: "2026-06-13T04:19:31.946598979+00:00"
---

Docs contract defines unified evidence record workflow: evidence record replaces split add/capture operator choice; target syntax is issue/<id>; command transcript behavior preserves command, exit status, success, timestamp, and bounded stdout/stderr; structured schema covers targets, proof_scope, kind, result, commands/artifacts, agent identity, independence_level, residual_risks, and follow_up_ids; parent coverage summaries and legacy prose migration expectations documented. Checks passed: git diff --check -- '*.md'; atelier lint; atelier export --check.
