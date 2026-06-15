---
created_at: "2026-06-15T05:51:13.303013156+00:00"
id: "atelier-npel"
evidence_type: "validation"
captured_at: "2026-06-15T05:51:13.302985381+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-c0f1"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-c0f1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Removed issue create --work and session/current-work write path. Proof: target/debug/atelier issue create --help omits --work and current-session wording; target/debug/atelier issue create \"Proof issue\" --work rejects unexpected --work; focused CLI test proves plain issue creation leaves active work none and no work_associations row; atelier lint atelier-c0f1, export --check, git diff --check passed."
updated_at: "2026-06-15T05:51:15.224611898+00:00"
---

Removed issue create --work and session/current-work write path. Proof: target/debug/atelier issue create --help omits --work and current-session wording; target/debug/atelier issue create "Proof issue" --work rejects unexpected --work; focused CLI test proves plain issue creation leaves active work none and no work_associations row; atelier lint atelier-c0f1, export --check, git diff --check passed.
