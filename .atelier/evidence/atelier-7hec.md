---
created_at: "2026-06-12T21:18:59.703785773+00:00"
id: "atelier-7hec"
evidence_type: "validation"
captured_at: "2026-06-12T21:18:59.252740273+00:00"
command: "target/debug/atelier workflow validate issue atelier-7yen --validator durable_state_current --validator issue_sections_parseable --validator evidence_attached --validator no_open_blockers --validator no_blocking_lints"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7yen"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier workflow validate issue atelier-7yen passed"
updated_at: "2026-06-12T21:19:01.229085066+00:00"
---

atelier workflow validate issue atelier-7yen passed

Command: target/debug/atelier workflow validate issue atelier-7yen --validator durable_state_current --validator issue_sections_parseable --validator evidence_attached --validator no_open_blockers --validator no_blocking_lints
Exit status: 0

Stdout summary:
Lint passed.
Workflow Validation: issue atelier-7yen
======================================
Transition: close
Validators: 5
Results
-------
  pass  durable_state_current
      Reason: canonical export is current
      Warning: validator took 242ms; validators should stay under 100ms
  pass  issue_sections_parseable
      Reason: parsed required sections Description, Outcome, Evidence are present and non-empty for 1 issue(s)
  pass  evidence_attached
      Reason: validating evidence is linked
  pass  no_open_blockers
      Reason: no open blockers
  pass  no_blocking_lints
      Reason: lint passed
      Warning: validator took 132ms; validators should stay under 100ms

Stderr summary:
(none)

